use crate::errors::CoggeratorError;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overviews_options() {
        let options = vec!["AUTO", "IGNORE_EXISTING", "FORCE_USE_EXISTING", "NONE"];

        options.iter().for_each(|option| {
            let overviews_option = OverviewsOption::new(option).unwrap();
            assert_eq!(overviews_option.to_creation_option().key, "OVERVIEWS");
            assert_eq!(overviews_option.to_creation_option().value, *option);
        });
    }

    #[test]
    fn test_overviews_invalid() {
        // Test that value throws an error
        assert!(OverviewsOption::new("INVALID").is_err());
        let e = OverviewsOption::new("INVALID").unwrap_err();
        assert_eq!(e.to_string(), "INVALID is not a valid OVERVIEWS option");
    }
}