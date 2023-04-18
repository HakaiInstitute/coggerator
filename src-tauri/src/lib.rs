use std::path::PathBuf;

use gdal::raster;

pub use errors::CoggeratorError;
pub use options::{BigTiffOption, CompressionOption, OverviewsOption, ResamplingOption};

mod errors;
mod options;

pub struct Args {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub no_data_value: Option<f64>,
    pub compression: CompressionOption,
    pub big_tiff: BigTiffOption,
    pub resampling: ResamplingOption,
    pub overviews: OverviewsOption,
}

impl Args {
    pub fn new(
        input_path: PathBuf,
        output_path: PathBuf,
        no_data_value: Option<f64>,
        compression: Option<&str>,
        big_tiff: Option<&str>,
        resampling: Option<&str>,
        overviews: Option<&str>,
    ) -> Result<Self, CoggeratorError> {
        let input_exists = input_path.try_exists()?;
        if !input_exists {
            return Err(CoggeratorError::PathError(format!(
                "Input path does not exist: {}",
                input_path.to_string_lossy()
            )));
        }
        let output_path_parent = match output_path.parent() {
            Some(p) => p.to_path_buf(),
            None => {
                return Err(CoggeratorError::PathError(String::from(
                    "Output path has no parent directory",
                )));
            }
        };
        let output_parent_exists = output_path_parent.try_exists()?;
        if !output_parent_exists {
            return Err(CoggeratorError::PathError(String::from(
                "Output path parent does not exist",
            )));
        }

        let compression = match compression {
            Some(c) => CompressionOption::new(c),
            None => Ok(CompressionOption::LZW),
        }?;

        let big_tiff = match big_tiff {
            Some(b) => BigTiffOption::new(b),
            None => Ok(BigTiffOption::IfSafer),
        }?;

        let resampling = match resampling {
            Some(r) => ResamplingOption::new(r),
            None => Ok(ResamplingOption::Cubic),
        }?;

        let overviews = match overviews {
            Some(o) => OverviewsOption::new(o),
            None => Ok(OverviewsOption::IgnoreExisting),
        }?;

        Ok(Self {
            input_path,
            output_path,
            no_data_value,
            compression,
            big_tiff,
            resampling,
            overviews,
        })
    }
}

pub fn convert_cog(args: Args) -> Result<PathBuf, CoggeratorError> {
    let dset = gdal::Dataset::open(args.input_path)?;

    // drivers
    let cog_driver = gdal::DriverManager::get_driver_by_name("COG")?;
    let vrt_driver = gdal::DriverManager::get_driver_by_name("VRT")?;

    // options
    let options = [
        raster::RasterCreationOption {
            key: "NUM_THREADS",
            value: "ALL_CPUS",
        },
        raster::RasterCreationOption {
            key: "BLOCKSIZE",
            value: "512",
        },
        args.resampling.to_creation_option(),
        args.big_tiff.to_creation_option(),
        args.compression.to_creation_option(),
        args.overviews.to_creation_option(),
    ];

    // Set nodata value on a VRTe
    let tmp_file = tempfile::NamedTempFile::new()?;
    let vrt = dset.create_copy(&vrt_driver, tmp_file.path(), &options)?;

    let num_bands = vrt.raster_count();
    for i in 1..=num_bands {
        let mut band = vrt.rasterband(i)?;
        band.set_no_data_value(args.no_data_value)?;
    }

    // Create COG
    vrt.create_copy(&cog_driver, &args.output_path, &options)?;

    Ok(args.output_path)
}
