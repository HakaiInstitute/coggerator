use crate::errors::CoggeratorError;

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
            "IfNeeded" => Ok(BigTiffOption::IfNeeded),
            "IfSafer" => Ok(BigTiffOption::IfSafer),
            _ => Err(CoggeratorError::InvalidBigTiffOption(s.to_string())),
        }
    }

    pub fn to_creation_option(&self) -> gdal::raster::RasterCreationOption {
        let value = match self {
            BigTiffOption::Yes => "YES",
            BigTiffOption::No => "NO",
            BigTiffOption::IfNeeded => "IfNeeded",
            BigTiffOption::IfSafer => "IfSafer",
        };
        gdal::raster::RasterCreationOption {
            key: "COMPRESS",
            value,
        }
    }
}
