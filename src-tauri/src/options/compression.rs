use crate::CoggeratorError;

pub enum CompressionOption {
    ZSTD,
    LZW,
    Deflate,
    Packbits,
    LZMA,
    LERC,
    LERCDeflate,
    LERCZSTD,
    None,
}

impl CompressionOption {
    pub fn new(s: &str) -> Result<Self, CoggeratorError> {
        match s {
            "ZSTD" => Ok(CompressionOption::ZSTD),
            "LZW" => Ok(CompressionOption::LZW),
            "DEFLATE" => Ok(CompressionOption::Deflate),
            "PACKBITS" => Ok(CompressionOption::Packbits),
            "LZMA" => Ok(CompressionOption::LZMA),
            "LERC" => Ok(CompressionOption::LERC),
            "LERC_DEFLATE" => Ok(CompressionOption::LERCDeflate),
            "LERC_ZSTD" => Ok(CompressionOption::LERCZSTD),
            "NONE" => Ok(CompressionOption::None),
            _ => Err(CoggeratorError::InvalidCompressionOption(s.to_string())),
        }
    }
    pub fn to_creation_option(&self) -> gdal::raster::RasterCreationOption {
        let value = match self {
            CompressionOption::ZSTD => "ZSTD",
            CompressionOption::LZW => "LZW",
            CompressionOption::Deflate => "DEFLATE",
            CompressionOption::Packbits => "PACKBITS",
            CompressionOption::LZMA => "LZMA",
            CompressionOption::LERC => "LERC",
            CompressionOption::LERCDeflate => "LERC_DEFLATE",
            CompressionOption::LERCZSTD => "LERC_ZSTD",
            CompressionOption::None => "NONE",
        };

        gdal::raster::RasterCreationOption {
            key: "COMPRESS",
            value,
        }
    }
}
