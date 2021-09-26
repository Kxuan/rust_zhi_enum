extern crate proc_macro;
use primitive_enum_macro::{EnumConvert, EnumTryConvert};

#[derive(EnumConvert)]
#[repr(u8)]
enum NumberConvert {
    Zero,
    One,
    Two,
    Three,
    Four,
    #[primitive_enum(unknown)]
    Unknown(u8),
}

#[derive(EnumTryConvert)]
#[repr(u8)]
enum NumberTryConvert {
    Zero,
    One,
    Two,
    Three,
    Four,
    #[primitive_enum(unknown)]
    Unknown(u8),
}


#[test]
fn test() {
    assert_eq!(NumberConvert::Zero.into_u8(), 0u8)
}