use crate::CoggeratorError;

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
