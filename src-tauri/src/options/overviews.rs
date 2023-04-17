use std::str::FromStr;

use strum_macros::{Display, EnumString, IntoStaticStr};

use crate::errors::CoggeratorError;

#[derive(Debug, PartialEq, EnumString, IntoStaticStr, Display)]
pub enum OverviewsOption {
    #[strum(serialize = "AUTO")]
    Auto,

    #[strum(serialize = "IGNORE_EXISTING")]
    IgnoreExisting,

    #[strum(serialize = "FORCE_USE_EXISTING")]
    ForceUseExisting,

    #[strum(serialize = "NONE")]
    None,
}

impl OverviewsOption {
    pub fn new(s: &str) -> Result<Self, CoggeratorError> {
        OverviewsOption::from_str(s).map_err(|_| CoggeratorError::InvalidOverviewsOption(s.to_string()))
    }

    pub fn to_creation_option(&self) -> gdal::raster::RasterCreationOption {
        let value: &'static str = self.into();

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