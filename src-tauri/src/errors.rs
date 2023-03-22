use gdal::errors::GdalError;

#[derive(Debug, thiserror::Error)]
pub enum CoggeratorError {
    #[error("Path error: {0}")]
    PathError(String),
    #[error("{0} is not a valid BIFTIFF option")]
    InvalidBigTiffOption(String),
    #[error("{0} is not a valid COMPRESSION option")]
    InvalidCompressionOption(String),
    #[error("{0} is not a valid OVERVIEWS option")]
    InvalidOverviewsOption(String),
    #[error("{0} is not a valid RESAMPLING option")]
    InvalidResamplingOption(String),
    #[error("GDAL error: {0}")]
    GdalError(#[from] GdalError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl serde::Serialize for CoggeratorError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            CoggeratorError::PathError(e) => serializer.serialize_str(e),
            CoggeratorError::InvalidBigTiffOption(e) => serializer.serialize_str(e),
            CoggeratorError::InvalidCompressionOption(e) => serializer.serialize_str(e),
            CoggeratorError::InvalidOverviewsOption(e) => serializer.serialize_str(e),
            CoggeratorError::InvalidResamplingOption(e) => serializer.serialize_str(e),
            CoggeratorError::GdalError(e) => serializer.serialize_str(e.to_string().as_ref()),
            CoggeratorError::IoError(e) => serializer.serialize_str(e.to_string().as_ref()),
        }
    }
}
