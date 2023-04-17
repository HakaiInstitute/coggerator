use crate::errors::CoggeratorError;

#[derive(Debug, PartialEq)]
pub enum BigTiffOption {
    Yes,
    No,
    IfNeeded,
    IfSafer,
}

impl BigTiffOption {
    pub fn new(s: &str) -> Result<Self, CoggeratorError> {
        match s {
            "YES" => Ok(BigTiffOption::Yes),
            "NO" => Ok(BigTiffOption::No),
            "IF_NEEDED" => Ok(BigTiffOption::IfNeeded),
            "IF_SAFER" => Ok(BigTiffOption::IfSafer),
            _ => Err(CoggeratorError::InvalidBigTiffOption(s.to_string())),
        }
    }

    pub fn to_creation_option(&self) -> gdal::raster::RasterCreationOption {
        let value = match self {
            BigTiffOption::Yes => "YES",
            BigTiffOption::No => "NO",
            BigTiffOption::IfNeeded => "IF_NEEDED",
            BigTiffOption::IfSafer => "IF_SAFER",
        };
        gdal::raster::RasterCreationOption {
            key: "BIGTIFF",
            value,
        }
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