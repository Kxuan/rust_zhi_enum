extern crate proc_macro;
use num_enum::FromPrimitive;


use zhi_enum_derive::{EnumConvert, EnumTryConvert};

#[derive(EnumConvert)]
#[repr(u8)]
enum NumberConvert {
    Zero,
    One,
    Two,
    Three,
    Four,
    Ten = 10,
    Eleven,
    #[zhi_enum(unknown)]
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
    Ten = 10,
    Eleven,
    #[zhi_enum(unknown)]
    Unknown(u8),
}


#[test]
fn test() {
    assert_eq!(NumberConvert::Three.into_u8(), 3u8);
    assert_eq!(NumberTryConvert::Three.try_into_u8().unwrap(), 3u8);
    assert_eq!(NumberConvert::Ten.into_u8(), 10u8);
    assert_eq!(NumberTryConvert::Ten.try_into_u8().unwrap(), 10u8);
    assert_eq!(NumberConvert::Eleven.into_u8(), 11u8);
    assert_eq!(NumberTryConvert::Eleven.try_into_u8().unwrap(), 11u8);
}