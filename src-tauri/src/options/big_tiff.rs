use std::str::FromStr;

use strum_macros::{Display, EnumString, IntoStaticStr};

use crate::errors::CoggeratorError;

#[derive(Debug, PartialEq, EnumString, IntoStaticStr, Display)]
pub enum BigTiffOption {
    #[strum(serialize = "YES")]
    Yes,

    #[strum(serialize = "NO")]
    No,

    #[strum(serialize = "IF_NEEDED")]
    IfNeeded,

    #[strum(serialize = "IF_SAFER")]
    IfSafer,
}

impl BigTiffOption {
    pub fn new(s: &str) -> Result<Self, CoggeratorError> {
        BigTiffOption::from_str(s).map_err(|_| CoggeratorError::InvalidBigTiffOption(s.to_string()))
    }

    pub fn to_creation_option(&self) -> gdal::raster::RasterCreationOption {
        let value: &'static str = self.into();
        gdal::raster::RasterCreationOption { key: "BIGTIFF", value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_tiff_options() {
        let options = vec!["YES", "NO", "IF_NEEDED", "IF_SAFER"];

        options.iter().for_each(|option| {
            let big_tiff_option = BigTiffOption::new(option).unwrap();
            assert_eq!(big_tiff_option.to_creation_option().key, "BIGTIFF");
            assert_eq!(big_tiff_option.to_creation_option().value, *option);
        });
    }

    #[test]
    fn test_big_tiff_options_invalid() {
        // Test that value throws an error
        assert!(BigTiffOption::new("INVALID").is_err());
        let e = BigTiffOption::new("INVALID").unwrap_err();
        assert_eq!(e.to_string(), "INVALID is not a valid BIFTIFF option");
    }
}