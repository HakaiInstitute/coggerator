use crate::CoggeratorError;

#[derive(Debug, PartialEq)]
pub enum ResamplingOption {
    Nearest,
    Average,
    Bilinear,
    Cubic,
    CubicSpline,
    Lanczos,
    Mode,
    RMS,
}

impl ResamplingOption {
    pub fn new(s: &str) -> Result<Self, CoggeratorError> {
        match s {
            "NEAREST" => Ok(ResamplingOption::Nearest),
            "AVERAGE" => Ok(ResamplingOption::Average),
            "BILINEAR" => Ok(ResamplingOption::Bilinear),
            "CUBIC" => Ok(ResamplingOption::Cubic),
            "CUBICSPLINE" => Ok(ResamplingOption::CubicSpline),
            "LANCZOS" => Ok(ResamplingOption::Lanczos),
            "MODE" => Ok(ResamplingOption::Mode),
            "RMS" => Ok(ResamplingOption::RMS),
            _ => Err(CoggeratorError::InvalidResamplingOption(s.to_string())),
        }
    }

    pub fn to_creation_option(&self) -> gdal::raster::RasterCreationOption {
        let value = match self {
            ResamplingOption::Nearest => "NEAREST",
            ResamplingOption::Average => "AVERAGE",
            ResamplingOption::Bilinear => "BILINEAR",
            ResamplingOption::Cubic => "CUBIC",
            ResamplingOption::CubicSpline => "CUBICSPLINE",
            ResamplingOption::Lanczos => "LANCZOS",
            ResamplingOption::Mode => "MODE",
            ResamplingOption::RMS => "RMS",
        };
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