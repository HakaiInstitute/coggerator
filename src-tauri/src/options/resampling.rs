use std::str::FromStr;

use strum_macros::{Display, EnumString, IntoStaticStr};

use crate::CoggeratorError;

#[derive(Debug, PartialEq, EnumString, IntoStaticStr, Display)]
pub enum ResamplingOption {
    #[strum(serialize = "NEAREST")]
    Nearest,

    #[strum(serialize = "AVERAGE")]
    Average,

    #[strum(serialize = "BILINEAR")]
    Bilinear,

    #[strum(serialize = "CUBIC")]
    Cubic,

    #[strum(serialize = "CUBICSPLINE")]
    CubicSpline,

    #[strum(serialize = "LANCZOS")]
    Lanczos,

    #[strum(serialize = "MODE")]
    Mode,

    #[strum(serialize = "RMS")]
    RMS,
}

impl ResamplingOption {
    pub fn new(s: &str) -> Result<Self, CoggeratorError> {
        ResamplingOption::from_str(s).map_err(|_| CoggeratorError::InvalidResamplingOption(s.to_string()))
    }

    pub fn to_creation_option(&self) -> gdal::raster::RasterCreationOption {
        let value: &'static str = self.into();

        gdal::raster::RasterCreationOption {
            key: "RESAMPLING",
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resampling_options() {
        let options = vec!["NEAREST", "AVERAGE", "BILINEAR", "CUBIC", "CUBICSPLINE", "LANCZOS", "MODE", "RMS"];

        options.iter().for_each(|option| {
            let resampling_option = ResamplingOption::new(option).unwrap();
            assert_eq!(resampling_option.to_creation_option().key, "RESAMPLING");
            assert_eq!(resampling_option.to_creation_option().value, *option);
        });
    }

    #[test]
    fn test_resampling_options_invalid() {
        // Test that value throws an error
        assert!(ResamplingOption::new("INVALID").is_err());
        let e = ResamplingOption::new("INVALID").unwrap_err();
        assert_eq!(e.to_string(), "INVALID is not a valid RESAMPLING option");
    }
}