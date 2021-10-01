use zhi_enum::{EnumConvert, EnumTryConvert};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, EnumConvert)]
#[repr(u8)]
enum NumberConvert {
    Zero,
    One,
    Two,
    Three,
    Four,
    Ten = 10,
    Eleven,
    Twenty = 10 + 10,
    TwentyOne,
    #[zhi_enum(unknown)]
    Unknown(u8),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, EnumTryConvert)]
#[repr(u8)]
enum NumberTryConvert {
    Zero,
    One,
    Two,
    Three,
    Four,
    Ten = 10,
    Eleven,
    Twenty = 10 + 10,
    TwentyOne,
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
    assert_eq!(NumberConvert::TwentyOne.into_u8(), 21u8);
    assert_eq!(NumberTryConvert::TwentyOne.try_into_u8().unwrap(), 21u8);

    assert_eq!(NumberConvert::from(3u8), NumberConvert::Three);
    assert_eq!(NumberTryConvert::try_from(21u8).unwrap(), NumberTryConvert::TwentyOne)
}