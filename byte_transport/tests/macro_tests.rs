use byte_transport::{ByteDecode, ByteEncode, Decoder};

#[derive(ByteEncode, ByteDecode, PartialEq, Eq, Debug, Default)]
struct TestStruct {
    field_a: i32,
    field_b: i64,
    sub_struct: Option<SubStruct>,
}

#[derive(ByteEncode, ByteDecode, PartialEq, Eq, Debug)]
struct SubStruct {
    b: bool,
    integer_32: i32,
}

#[derive(ByteEncode, ByteDecode, PartialEq, Eq, Debug)]
enum TestEnum {
    A,
    B(i32, i32),
    C {
        test_field: i32,
        test_field2: bool,
    },
}

#[test]
fn macro_struct_test() -> Result<(), byte_transport::Error> {
    let test_struct = TestStruct {
        field_a: 0i32,
        field_b: 5i64,
        sub_struct: None
    };


    let mut bytes: Vec<u8> = Vec::new();
    test_struct.simple_encode(&mut bytes)?;
    let decoded_test_struct =  TestStruct::simple_decode(&mut Decoder::new(bytes))?;
    assert_eq!(test_struct, decoded_test_struct);

    Ok(())
}

#[test]
fn macro_enum_test() -> Result<(), byte_transport::Error> {
    let test_enum_a = TestEnum::A;
    let mut bytes_a: Vec<u8> = Vec::new();
    test_enum_a.simple_encode(&mut bytes_a)?;
    let decoded_test_enum_a = TestEnum::simple_decode(&mut Decoder::new(bytes_a))?;
    assert_eq!(test_enum_a, decoded_test_enum_a);

    let test_enum_b = TestEnum::B(0i32, 5i32);
    let mut bytes_b: Vec<u8> = Vec::new();
    test_enum_b.simple_encode(&mut bytes_b)?;
    let decoded_test_enum_b = TestEnum::simple_decode(&mut Decoder::new(bytes_b))?;
    assert_eq!(test_enum_b, decoded_test_enum_b);

    let test_enum_c = TestEnum::C {
        test_field: 0i32,
        test_field2: false
    };
    let mut bytes_c: Vec<u8> = Vec::new();
    test_enum_c.simple_encode(&mut bytes_c)?;
    let decoded_test_enum_c = TestEnum::simple_decode(&mut Decoder::new(bytes_c))?;
    assert_eq!(test_enum_c, decoded_test_enum_c);

    Ok(())
}
