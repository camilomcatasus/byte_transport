use std::{array::TryFromSliceError, time::Duration};
use core::fmt::Debug;

mod error;
pub use byte_transport_macros::{ByteEncode, ByteDecode};

#[cfg(feature = "bevy")]
use bevy::prelude::*;

pub use error::Error;

pub struct Decoder {
    pub index: usize,
    pub bytes: Vec<u8>,

}

impl Decoder {
    pub fn new(bytes: Vec<u8>) -> Self { Decoder { index: 0, bytes } }
}

pub trait ByteEncode {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error>;
}

impl ByteEncode for f64 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.extend_from_slice(&f64::to_le_bytes(*self));
        Ok(())
    }
}

impl ByteEncode for f32 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.extend_from_slice(&f32::to_le_bytes(*self));
        Ok(())
    }
}
impl ByteEncode for i64 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.extend_from_slice(&i64::to_le_bytes(*self));
        Ok(())
    }
}

impl ByteEncode for i32 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.extend_from_slice(&i32::to_le_bytes(*self));
        Ok(())
    }
}

impl ByteEncode for i16 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.extend_from_slice(&i16::to_le_bytes(*self));
        Ok(())
    }
}

impl ByteEncode for i8 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.extend_from_slice(&i8::to_le_bytes(*self));
        Ok(())
    }
}

impl ByteEncode for u128 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.extend_from_slice(&u128::to_le_bytes(*self));
        Ok(())
    }
}

impl ByteEncode for u64 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.extend_from_slice(&u64::to_le_bytes(*self));
        Ok(())
    }
}

impl ByteEncode for u32 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.extend_from_slice(&u32::to_le_bytes(*self));
        Ok(())
    }
}

impl ByteEncode for u16 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.extend_from_slice(&u16::to_le_bytes(*self));
        Ok(())
    }
}

impl ByteEncode for u8 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        bytes.push(*self);
        Ok(())
    }
}

impl<T: ByteEncode> ByteEncode for Vec<T> {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        let vec_length: u16 = self.len() as u16;
        println!("Encoded Vec Length: {}", vec_length);
        vec_length.simple_encode(bytes)?;
        for encodable in self {
            encodable.simple_encode(bytes)?;
        };
        Ok(())
    }
}

impl<T, const N: usize> ByteEncode for [T; N]
where
    T: ByteEncode,
{   
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        for index in 0..N {
            self[index].simple_encode(bytes)?;
        }
        Ok(())
    }
}

#[cfg(feature = "bevy")]
impl ByteEncode for Vec3 {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> anyhow::Result<()> {
        self.to_array().simple_encode(bytes)?;
        Ok(())
    }
}

#[cfg(feature = "bevy")]
impl ByteEncode for Quat {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> anyhow::Result<()> {
        self.to_array().simple_encode(bytes)?;
        println!("{:?}", bytes);
        Ok(())
    }
}

#[cfg(feature = "bevy")]
impl ByteEncode for Transform {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> anyhow::Result<()> {
        self.translation.simple_encode(bytes)?;
        self.rotation.simple_encode(bytes)?;
        self.scale.simple_encode(bytes)?;
        Ok(())
    }
}

pub trait ByteDecode {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self, Error>
    where Self: Sized;
}

impl<T: ByteDecode> ByteDecode for Vec<T> {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self, Error> {
        let mut temp_vec = Vec::new();

        let decode_length = u16::simple_decode(decoder)?;
        for _ in 0..decode_length {
            temp_vec.push(T::simple_decode(decoder)?);
        }

        Ok(temp_vec)
    }
}

impl ByteDecode for f64 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        let byte_slice_result: Result<[u8;8], TryFromSliceError> = decoder.bytes[decoder.index..(decoder.index + 8)].try_into();

        decoder.index += 8;
        match byte_slice_result {
            Ok(byte_slice) => return Ok(f64::from_le_bytes(byte_slice)),
            Err(_) => return Err(Error::SimpleDecodeTryFrom)
        }
    }
}

impl ByteDecode for f32 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        let byte_slice_result: Result<[u8;4], TryFromSliceError> = decoder.bytes[decoder.index..(decoder.index + 4)].try_into();

        decoder.index += 4;
        match byte_slice_result {
            Ok(byte_slice) => return Ok(f32::from_le_bytes(byte_slice)),
            Err(_) => return Err(Error::SimpleDecodeTryFrom)
        }
    }
}

impl ByteDecode for u128 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        let byte_slice_result: Result<[u8;16], TryFromSliceError> = decoder.bytes[decoder.index..(decoder.index + 16)].try_into();

        decoder.index += 16;
        match byte_slice_result {
            Ok(byte_slice) => return Ok(u128::from_le_bytes(byte_slice)),
            Err(_) => return Err(Error::SimpleDecodeTryFrom)
        }
    }
}

impl ByteDecode for u64 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        let byte_slice_result: Result<[u8;8], TryFromSliceError> = decoder.bytes[decoder.index..(decoder.index + 8)].try_into();

        decoder.index += 8;
        match byte_slice_result {
            Ok(byte_slice) => return Ok(u64::from_le_bytes(byte_slice)),
            Err(_) => return Err(Error::SimpleDecodeTryFrom)
        }
    }
}

impl ByteDecode for u32 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        let byte_slice_result: Result<[u8;4], TryFromSliceError> = decoder.bytes[decoder.index..(decoder.index + 4)].try_into();

        decoder.index += 4;
        match byte_slice_result {
            Ok(byte_slice) => return Ok(u32::from_le_bytes(byte_slice)),
            Err(_) => return Err(Error::SimpleDecodeTryFrom)
        }
    }
}

impl ByteDecode for u16 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        let byte_slice_result: Result<[u8;2], TryFromSliceError> = decoder.bytes[decoder.index..(decoder.index + 2)].try_into();

        decoder.index += 2;
        match byte_slice_result {
            Ok(byte_slice) => return Ok(u16::from_le_bytes(byte_slice)),
            Err(_) => return Err(Error::SimpleDecodeTryFrom)
        }
    }
}

impl ByteDecode for u8 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        let byte = decoder.bytes[decoder.index];
        decoder.index += 1;
        return Ok(byte);
    }
}

impl ByteDecode for i64 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error>
        where Self: Sized {
        
        let byte_slice_result: Result<[u8;8], TryFromSliceError > = decoder.bytes[decoder.index..(decoder.index + 8)].try_into();
        decoder.index += 8;
        match byte_slice_result {
            Ok(byte_slice) => return Ok(i64::from_le_bytes(byte_slice)),
            Err(_) => return Err(Error::SimpleDecodeTryFrom)
        }
    }
}

impl ByteDecode for i32 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        let byte_slice_result: Result<[u8;4], TryFromSliceError> = decoder.bytes[decoder.index..(decoder.index + 4)].try_into();

        decoder.index += 4;
        match byte_slice_result {
            Ok(byte_slice) => return Ok(i32::from_le_bytes(byte_slice)),
            Err(_) => return Err(Error::SimpleDecodeTryFrom)
        }
    }
}

impl ByteDecode for i16 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        let byte_slice_result: Result<[u8;2], TryFromSliceError> = decoder.bytes[decoder.index..(decoder.index + 2)].try_into();

        decoder.index += 2;
        match byte_slice_result {
            Ok(byte_slice) => return Ok(i16::from_le_bytes(byte_slice)),
            Err(_) => return Err(Error::SimpleDecodeTryFrom)
        }
    }
}

impl ByteDecode for i8 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        let byte = [decoder.bytes[decoder.index]];
        decoder.index += 1;
        return Ok(i8::from_le_bytes(byte));
    }
}

impl<T, const N: usize> ByteDecode for [T; N]
where
    T: ByteDecode + Debug,
{
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error>
        where Self: Sized {
        use std::mem::MaybeUninit;
        let mut arr: [T; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..N {
            arr[i] = T::simple_decode(decoder)?; // Create and place each element
        }
        // Transmute the MaybeUninit array into a fully initialized array
        Ok(arr)
    }
}

#[cfg(feature = "godot")]
pub mod godot {
    pub use super::{ByteDecode, ByteEncode, Decoder};

    use godot::prelude::*;

    impl ByteEncode for Vector3 {
        fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), crate::Error> {
            self.x.simple_encode(bytes)?;
            self.y.simple_encode(bytes)?;
            self.z.simple_encode(bytes)?;

            Ok(())
        }
    }

    impl ByteDecode for Vector3 {
        fn simple_decode(decoder: &mut Decoder) -> Result<Self, crate::Error> {
            Ok(Vector3 {
                x: f32::simple_decode(decoder)?,
                y: f32::simple_decode(decoder)?,
                z: f32::simple_decode(decoder)?
            })
        }
    }

    impl ByteEncode for Vector2 {
        fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), crate::Error> {
            self.x.simple_encode(bytes)?;
            self.y.simple_encode(bytes)?;

            Ok(())
        }
    }

    impl ByteDecode for Vector2 {
        fn simple_decode(decoder: &mut Decoder) -> Result<Self, crate::Error> {
            Ok(Vector2 {
                x: f32::simple_decode(decoder)?,
                y: f32::simple_decode(decoder)?,
            })
        }
    }

    impl ByteEncode for Quaternion {
        fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), crate::Error> {
            self.x.simple_encode(bytes)?;
            self.y.simple_encode(bytes)?;
            self.z.simple_encode(bytes)?;
            self.w.simple_encode(bytes)?;

            Ok(())
        }
    }

    impl ByteDecode for Quaternion {
        fn simple_decode(decoder: &mut Decoder) -> Result<Self, crate::Error>
            where Self: Sized {
            Ok(Quaternion {
                x: f32::simple_decode(decoder)?,
                y: f32::simple_decode(decoder)?,
                z: f32::simple_decode(decoder)?,
                w: f32::simple_decode(decoder)?,
            })
        }
    }


}

#[cfg(feature = "bevy")]
impl ByteDecode for Quat {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        type Float4 = [f32;4];
        let quat_bytes = Float4::simple_decode(decoder)?;

        Ok(Quat::from_array(quat_bytes))
    }
}

#[cfg(feature = "bevy")]
impl ByteDecode for Vec3 {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        Ok(Vec3 {
            x: f32::simple_decode(decoder)?,
            y: f32::simple_decode(decoder)?,
            z: f32::simple_decode(decoder)?,
        })
    }
}

#[cfg(feature = "bevy")]
impl ByteDecode for Transform {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error> {
        Ok(Transform {
            translation: Vec3::simple_decode(decoder)?,
            rotation: Quat::simple_decode(decoder)?,
            scale: Vec3::simple_decode(decoder)?
        })
    }
}

impl<T: Sized + ByteDecode> ByteDecode for Option<T> {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error>
        where Self: Sized {
        match u8::simple_decode(decoder)? {
            SOME_FLAG => Ok(Some(T::simple_decode(decoder)?)),
            NONE_FLAG => Ok(None),
            _ => Err(Error::SimpleDecodeError("Parse Error, Option flag not found".to_string()))
        }
    }
}

impl<T: Sized + ByteEncode> ByteEncode for Option<T> {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        match self {
            Some(val) => {
                SOME_FLAG.simple_encode(bytes)?;
                val.simple_encode(bytes)?;
            }
            None => {
                NONE_FLAG.simple_encode(bytes)?;
            }
        }
        Ok(())
    }
}

impl ByteEncode for bool {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        match self {
            true => 1u8.simple_encode(bytes),
            false => 0u8.simple_encode(bytes),
        }
    }
}

impl ByteDecode for bool {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error>
        where Self: Sized {
        match u8::simple_decode(decoder)? {
            0u8 => Ok(false),
            1u8 => Ok(true),
            _ => Err(Error::SimpleDecodeError("Error parsing bool".to_string())),
        }
    }
}

impl ByteEncode for Duration {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        self.as_secs().simple_encode(bytes)
    }
}

impl ByteDecode for Duration {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self,Error>
        where Self: Sized {
        Ok(Duration::from_secs(u64::simple_decode(decoder)?))
    }
}

impl ByteEncode for String {
    fn simple_encode(&self, bytes:&mut Vec<u8>) -> Result<(), Error> {
        let str_bytes = self.as_bytes();
        let byte_len = u64::try_from(str_bytes.len()).unwrap();
        byte_len.simple_encode(bytes);
        bytes.extend_from_slice(str_bytes);
        Ok(())
    }
}

impl ByteDecode for String {
    fn simple_decode(decoder: &mut Decoder) -> Result<Self, Error>
        where Self: Sized {
        let byte_len = usize::try_from(u64::simple_decode(decoder)?).unwrap();
        let str = String::from_utf8_lossy(&decoder.bytes[decoder.index..(decoder.index + byte_len)]);

        decoder.index += byte_len;

        Ok(str.into())
    }
}

const SOME_FLAG: u8 = 1u8;
const NONE_FLAG: u8 = 0u8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64_transport() -> Result<(), Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let test_val = 5u64;
        test_val.simple_encode(&mut bytes)?;
        let mut decoder = Decoder {
            index: 0,
            bytes,
        };
        let decoded_val = u64::simple_decode(&mut decoder)?;

        assert!(test_val == decoded_val, "Test Val: {} does not match Decoded Val: {}", test_val, decoded_val);
        Ok(())
    }

    #[test]
    fn f32_transport() -> Result<(), Error> {
        let mut bytes: Vec<u8> = Vec::new();
        let test_val = 2.55f32;
        test_val.simple_encode(&mut bytes)?;
        let mut decoder = Decoder {
            index: 0,
            bytes,
        };
        let decoded_val = f32::simple_decode(&mut decoder)?;

        assert!(test_val == decoded_val, "Test Val: {} does not match Decoded Val: {}", test_val, decoded_val);
        Ok(())
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn vec3_transport() -> anyhow::Result<()> {
        let mut bytes: Vec<u8> = Vec::new();
        let test_val = Vec3 {
            x: 5f32,
            y: 2f32,
            z: 2.5f32,
        };
        test_val.simple_encode(&mut bytes)?;
        let mut decoder = Decoder {
            index: 0,
            bytes,
        };
        let decoded_val = Vec3::simple_decode(&mut decoder)?;

        assert!(test_val == decoded_val, "Test Val: {} does not match Decoded Val: {}", test_val, decoded_val);
        Ok(())
    }

    #[test]
    #[cfg(feature = "bevy")]
    fn quat_transport() -> anyhow::Result<()> {
        let mut bytes: Vec<u8> = Vec::new();
        let test_val = Quat::from_xyzw(
            5.5f32,
            32f32,
            0.143f32,
            1f32
        );
        test_val.simple_encode(&mut bytes)?;
        let mut decoder = Decoder {
            index: 0,
            bytes,
        };
        let decoded_val = Quat::simple_decode(&mut decoder)?;

        assert!(test_val == decoded_val, "Test Val: {} does not match Decoded Val: {}", test_val, decoded_val);
        Ok(())
    }

    #[test]
    #[cfg(feature = "bevy")]
    fn transform_transport() -> anyhow::Result<()> {
        let mut bytes: Vec<u8> = Vec::new();
        let test_val = Transform::default();
        test_val.simple_encode(&mut bytes)?;
        let mut decoder = Decoder {
            index: 0,
            bytes,
        };
        let decoded_val = Transform::simple_decode(&mut decoder)?;

        assert!(test_val == decoded_val, "Test Val: {:?} does not match Decoded Val: {:?}", test_val, decoded_val);
        Ok(())
    }

    #[test]
    fn simple_vector() -> Result<(), Error> {
        let mut bytes: Vec<u8> = Vec::new();
        type ValType = Vec<u64>;
        let test_val: Vec<u64> = vec![
            5,
            3,
            3,
            9
        ];

        test_val.simple_encode(&mut bytes)?;
        let mut decoder = Decoder {
            index: 0,
            bytes,
        };
        let decoded_val = ValType::simple_decode(&mut decoder)?;
        assert!(test_val == decoded_val, "Test Val: {:?} does not match Decoded Val: {:?}", test_val, decoded_val);
        Ok(())
    }
}

