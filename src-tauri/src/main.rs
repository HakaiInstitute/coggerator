// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::path::PathBuf;

use coggerator::{convert_cog, Args, CoggeratorError};

#[tauri::command]
async fn convert(
    input_path: PathBuf,
    output_path: PathBuf,
    no_data_value: Option<f64>,
    compression: Option<&str>,
    big_tiff: Option<&str>,
    resampling: Option<&str>,
    overviews: Option<&str>,
) -> Result<PathBuf, CoggeratorError> {
    // Append _cog.tif to input_path to make output_path
    let args = Args::new(
        input_path,
        output_path,
        no_data_value,
        compression,
        big_tiff,
        resampling,
        overviews,
    )?;
    let created_cog = convert_cog(args)?;
    Ok(created_cog)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
