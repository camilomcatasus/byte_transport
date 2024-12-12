extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields};

// Derive macro for ByteEncode
#[proc_macro_derive(ByteEncode)]
pub fn derive_byte_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let encode_impl = match input.data {
        Data::Struct(ref data) => {
            let field_encodes = data.fields.iter().map(|f| {
                let name = &f.ident;
                quote! {
                    ByteEncode::simple_encode(&self.#name, bytes)?;
                }
            });
            quote! {
                impl ByteEncode for #name {
                    fn simple_encode(&self, bytes: &mut Vec<u8>) -> Result<(), byte_transport::Error> {
                        #(#field_encodes)*
                        Ok(())
                    }
                }
            }
        },
        Data::Enum(ref data_enum) => {
            let variant_encodes = data_enum.variants.iter().enumerate().map(|(idx, variant)| {
                let variant_name = &variant.ident;
                let idx = idx as u8;  // Enum variant index as a `u8`
                match variant.fields {
                    Fields::Unit => quote! {
                        Self::#variant_name => {
                            bytes.push(#idx);
                            Ok(())
                        }
                    },
                    Fields::Unnamed(ref fields) => {

                        let field_names = fields.unnamed.iter().enumerate().map(|(i, _)| {
                            let field_name = syn::Ident::new(&format!("field_{i}"), proc_macro2::Span::call_site());
                            quote! {
                                ref #field_name 
                            }
                        });

                        let field_encodes = fields.unnamed.iter().enumerate().map(|(i, _)| {
                            let field_name = syn::Ident::new(&format!("field_{i}"), proc_macro2::Span::call_site());
                            quote! {
                                ByteEncode::simple_encode(#field_name, bytes)?;
                            }
                        });
                        let encode_tokens = quote! {
                            Self::#variant_name(#(#field_names),*) => {
                                bytes.push(#idx); 
                                #(#field_encodes)*
                                Ok(())
                            }
                        };

                        println!("{encode_tokens}");

                        encode_tokens
                    },
                    Fields::Named(ref named_fields) => {
                        let struct_field_names = named_fields.named.iter().map(|field| {
                            let ident = &field.ident;
                            quote! {
                                ref #ident
                            }
                        });

                        let encode_fields = named_fields.named.iter().map(|field| {
                            let ident = &field.ident;
                            quote! {
                                ByteEncode::simple_encode(#ident, bytes)?;
                            }
                        });

                        let encode_tokens = quote! {
                            Self::#variant_name{#(#struct_field_names),*} => {
                                bytes.push(#idx); 
                                #(#encode_fields)*
                                Ok(())
                            }
                        };

                        println!("{encode_tokens}");

                        encode_tokens
                    },
                }
            });
            quote! {
                impl ByteEncode for #name {
                    fn simple_encode(&self, bytes: &mut Vec<u8>) -> Result<(), byte_transport::Error> {
                        match *self {
                            #(#variant_encodes,)*
                        }
                    }
                }
            }
        },
        _ => panic!("ByteEncode can only be derived for structs and enums."),
    };

    TokenStream::from(encode_impl)
}

// Derive macro for ByteDecode
#[proc_macro_derive(ByteDecode)]
pub fn derive_byte_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let decode_impl = match input.data {
        Data::Struct(ref data) => {
            let field_decodes = data.fields.iter().map(|f| {
                let name = &f.ident;
                quote! {
                    let #name = ByteDecode::simple_decode(decoder)?;
                }
            });
            let field_names = data.fields.iter().map(|f| {
                let name = &f.ident;
                quote! { #name }
            });
            quote! {
                impl ByteDecode for #name {
                    fn simple_decode(decoder: &mut byte_transport::Decoder) -> Result<Self, byte_transport::Error> {
                        #(#field_decodes)*
                        Ok(Self {
                            #(#field_names),*
                        })
                    }
                }
            }
        },
         // Enum handling
        Data::Enum(ref data_enum) => {
            let variant_decodes = data_enum.variants.iter().enumerate().map(|(idx, variant)| {
                let variant_name = &variant.ident;
                let idx = idx as u8;  // Enum variant index as a `u8`
                match variant.fields {
                    Fields::Unit => quote! {
                        #idx => Ok(Self::#variant_name),
                    },
                    Fields::Unnamed(ref fields) => {
                        println!("Fields {:?}", fields.to_token_stream());
                        let field_decodes = fields.unnamed.iter().enumerate().map(|(i, field)| {
                            let field_ident = syn::Ident::new(&format!("field_{i}"), proc_macro2::Span::call_site());
                            
                            quote! {
                                let #field_ident = #field::simple_decode(decoder)?;
                            }
                        });

                        let field_names = fields.unnamed.iter().enumerate().map(|(i, _)| {
                            syn::Ident::new(&format!("field_{i}"), proc_macro2::Span::call_site())
                        });

                        let stream = quote! {
                            #idx => {
                                #(#field_decodes)*
                                Ok(Self::#variant_name(#(#field_names),*))
                            }
                        };
                        println!("Unnamed Fields result: {stream}");
                        stream
                    },
                    Fields::Named(ref named_fields) => {
                        let field_decodes = named_fields.named.iter().map(|named_field| {
                            let field_ident = &named_field.ident;
                            let field_type = &named_field.ty;
                            quote! {
                                #field_ident: #field_type::simple_decode(decoder)?,
                            }
                        });

                        let stream = quote! {
                            #idx => {
                                Ok(Self::#variant_name {
                                    #(#field_decodes)*
                                })
                            }
                        };

                        println!("Byte Decode implementation: {stream}");
                        stream
                    },
                }
            });
            quote! {
                impl ByteDecode for #name {
                    fn simple_decode(decoder: &mut byte_transport::Decoder) -> Result<Self,byte_transport::Error> {
                        let variant_idx = u8::simple_decode(decoder)?;
                        match variant_idx {
                            #(#variant_decodes)*
                            _ => Err(byte_transport::Error::DecodingEnumVariant(variant_idx)),
                        }
                    }
                }
            }
        },
        _ => panic!("ByteDecode can only be derived for structs and enums."),    };

    TokenStream::from(decode_impl)
}
