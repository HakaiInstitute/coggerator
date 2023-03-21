use crate::errors::CoggeratorError;

pub enum OverviewsOption {
    Auto,
    IgnoreExisting,
    ForceUseExisting,
    None,
}

impl OverviewsOption {
    pub fn new(s: &str) -> Result<OverviewsOption, CoggeratorError> {
        match s {
            "AUTO" => Ok(OverviewsOption::Auto),
            "IGNORE_EXISTING" => Ok(OverviewsOption::IgnoreExisting),
            "FORCE_USE_EXISTING" => Ok(OverviewsOption::ForceUseExisting),
            "NONE" => Ok(OverviewsOption::None),
            _ => Err(CoggeratorError::InvalidOverviewsOption(s.to_string())),
        }
    }

    pub fn to_creation_option(&self) -> gdal::raster::RasterCreationOption {
        let value = match self {
            OverviewsOption::Auto => "AUTO",
            OverviewsOption::IgnoreExisting => "IGNORE_EXISTING",
            OverviewsOption::ForceUseExisting => "FORCE_USE_EXISTING",
            OverviewsOption::None => "NONE",
        };
        gdal::raster::RasterCreationOption {
            key: "OVERVIEWS",
            value,
        }
    }
}
