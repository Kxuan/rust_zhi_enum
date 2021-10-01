# Zhi Enum

Easily create casting traits with unknown field support.

# Usage

1. Add `EnumConvert` or `EnumTryConvert` into the `derive` attribute.

```rust
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

```

2. Then, you can use `.from`/`.into` convert functions

```rust
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
```

## Handle unknown discriminant

* If you have defined a variant with `#[zhi_enum(unknown)]` attribute in your enum like this :
  ```rust
  #[zhi_enum(unknown)]
  Whatever(repr type),
  ```
  which the `repr type` is the represented type as you define use `#[repr(...)]`.

  Then, when you convert a repr type into enum type, an unknown discriminant will be converted
  to this variant with its unknown value.

* If you have not defined, the `.try_into()` will returns `Err(UnknownVariantError{})`, and the
  `.into()` will call `panic!`.

# FAQ
* Error E0658

  If you get this error: 
  ```
  error[E0658]: custom discriminant values are not allowed in enums with tuple or struct variants
    --> <source>:2:9
  ```
  It means that your rustc version is lower than `1.56`.
  
  You can just upgrade your rustc. Or if you don't want upgrade, you have to add `#![feature(arbitrary_enum_discriminant)]` to your project.


## License
BSD-3-Clause
