extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

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
                    fn simple_encode(&self, bytes: &mut Vec<u8>) -> anyhow::Result<()> {
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
                        let field_encodes = fields.unnamed.iter().enumerate().map(|(i, _)| {
                            let index = syn::Index::from(i);
                            quote! {
                                ByteEncode::simple_encode(&self.#index, bytes)?;
                            }
                        });
                        quote! {
                            Self::#variant_name(ref data) => {
                                bytes.push(#idx);
                                #(#field_encodes)*
                                Ok(())
                            }
                        }
                    },
                    Fields::Named(_) => unimplemented!("Named fields in enums are not supported yet"),
                }
            });
            quote! {
                impl ByteEncode for #name {
                    fn simple_encode(&self, bytes: &mut Vec<u8>) -> anyhow::Result<()> {
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
                    fn simple_decode(decoder: &mut Decoder) -> anyhow::Result<Self> {
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
                        let field_decodes = fields.unnamed.iter().map(|_| {
                            quote! {
                                let field = ByteDecode::simple_decode(decoder)?;
                            }
                        });
                        let field_names = fields.unnamed.iter().enumerate().map(|(i, _)| {
                            let index = syn::Index::from(i);
                            quote! { field }
                        });
                        quote! {
                            #idx => {
                                #(#field_decodes)*
                                Ok(Self::#variant_name(#(#field_names),*))
                            }
                        }
                    },
                    Fields::Named(_) => unimplemented!("Named fields in enums are not supported yet"),
                }
            });
            quote! {
                impl ByteDecode for #name {
                    fn simple_decode(decoder: &mut Decoder) -> anyhow::Result<Self> {
                        let variant_idx = u8::simple_decode(decoder)?;
                        match variant_idx {
                            #(#variant_decodes)*
                            _ => Err(anyhow::anyhow!("Invalid enum variant index")),
                        }
                    }
                }
            }
        },
        _ => panic!("ByteDecode can only be derived for structs and enums."),    };

    TokenStream::from(decode_impl)
}
