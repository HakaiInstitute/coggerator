use std::str::FromStr;

use strum_macros::{Display, EnumString, IntoStaticStr};

use crate::CoggeratorError;

#[derive(Debug, PartialEq, EnumString, IntoStaticStr, Display)]
pub enum CompressionOption {
    #[strum(serialize = "ZSTD")]
    ZSTD,

    #[strum(serialize = "LZW")]
    LZW,

    #[strum(serialize = "DEFLATE")]
    Deflate,

    #[strum(serialize = "PACKBITS")]
    Packbits,

    #[strum(serialize = "LZMA")]
    LZMA,

    #[strum(serialize = "LERC")]
    LERC,

    #[strum(serialize = "LERC_DEFLATE")]
    LERCDeflate,

    #[strum(serialize = "LERC_ZSTD")]
    LERCZSTD,

    #[strum(serialize = "NONE")]
    None,
}

impl CompressionOption {
    pub fn new(s: &str) -> Result<Self, CoggeratorError> {
        CompressionOption::from_str(s).map_err(|_| CoggeratorError::InvalidCompressionOption(s.to_string()))
    }

    pub fn to_creation_option(&self) -> gdal::raster::RasterCreationOption {
        let value: &'static str = self.into();
        gdal::raster::RasterCreationOption { key: "COMPRESS", value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_options() {
        let options = vec!["ZSTD", "LZW", "DEFLATE", "PACKBITS", "LZMA", "LERC", "LERC_DEFLATE", "LERC_ZSTD", "NONE"];

        options.iter().for_each(|option| {
            let compression_option = CompressionOption::new(option).unwrap();
            assert_eq!(compression_option.to_creation_option().key, "COMPRESS");
            assert_eq!(compression_option.to_creation_option().value, *option);
        });
    }

    #[test]
    fn test_compression_options_invalid() {
        // Test that value throws an error
        assert!(CompressionOption::new("INVALID").is_err());
        let e = CompressionOption::new("INVALID").unwrap_err();
        assert_eq!(e.to_string(), "INVALID is not a valid COMPRESSION option");
    }
}
