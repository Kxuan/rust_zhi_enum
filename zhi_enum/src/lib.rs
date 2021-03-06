pub use zhi_enum_derive::{EnumConvert, EnumTryConvert};

#[derive(Debug, Clone)]
pub struct UnknownVariantError {}

impl core::fmt::Display for UnknownVariantError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "unknown variant")
    }
}

impl std::error::Error for UnknownVariantError {}

