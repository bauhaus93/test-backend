use std::collections::BTreeMap;
use std::fs;

use hyper::http::StatusCode;

use super::{ StaticResponse, ApplicationError };

pub type AssetMap = BTreeMap<String, StaticResponse>;

pub fn load_assets(asset_folder: &str) -> Result<AssetMap, ApplicationError> {
    info!("Loading assets: folder = '{}'", asset_folder);
    let mut asset_map = BTreeMap::new();
    let file_entries = fs::read_dir(asset_folder)?;
    for dir_entry in file_entries {
        match dir_entry {
            Ok(file_entry) => {
                if is_valid_file(&file_entry) {
                    load_asset(&file_entry, &mut asset_map)?;
                }
            },
            Err(e) => return Err(e.into())
        }
    }
    Ok(asset_map)
}

fn load_asset(entry: &fs::DirEntry, asset_map: &mut AssetMap) -> Result<(), ApplicationError> {
    trace!("Loading next asset...");
    if let Some(file_path) = extract_filepath(entry) {
        if let Some(asset_name) = extract_asset_name(entry) {
            let content_type = determine_content_type(entry);
            let asset = StaticResponse::from_file(&file_path, StatusCode::OK, content_type)?;
            info!("Adding asset: name = '{}', content type = '{}', file = '{}'", asset_name, content_type, file_path);
            if let Some(_existing_asset) = asset_map.insert(asset_name.clone(), asset) {    // TODO: maybe remove clone, change warn!(..)
                warn!("Asset with name '{}' was already existing!", asset_name);
            }
        }
    }
    Ok(())
}

fn extract_asset_name(entry: &fs::DirEntry) -> Option<String> {
    match extract_filename(entry) {
        Some(file_name) => {
            Some(file_name.chars()
                .filter_map(|c| {
                    match c {
                        '_' => Some('-'),
                        '.' => Some('-'),
                        c if c.is_ascii_alphanumeric() => Some(c),
                        _ => None
                    }
                }).collect::<String>())
        },
        None => None
    }
}

fn determine_content_type(entry: &fs::DirEntry) -> &'static str {
    const DEFAULT_CONTENT_TYPE: &str = "application/octet-stream";
    match entry.path().extension() {
        Some(os_ext) => {
            match os_ext.to_str() {
                Some(ext) => {
                    match ext {
                        "html" => "text/html",
                        "css" => "text/css",
                        "js" => "application/javascript",
                        _ => {
                            warn!("File with unhandled extension: '{}', setting content type to '{}'", ext, DEFAULT_CONTENT_TYPE);
                            DEFAULT_CONTENT_TYPE
                        }
                    }
                },
                None => {
                    warn!("Could not convert os str of file extension, setting content type to '{}'.", DEFAULT_CONTENT_TYPE);
                    DEFAULT_CONTENT_TYPE  
                }
            }
        },
        None => {
            warn!("File has not extension, setting content type to '{}'.", DEFAULT_CONTENT_TYPE);
            DEFAULT_CONTENT_TYPE
        }
    }
}

fn extract_filename(entry: &fs::DirEntry) -> Option<String> {
    match entry.file_name().to_str() {
        Some(file_name) => Some(file_name.to_owned()),
        None => None
    }
}

fn extract_filepath(entry: &fs::DirEntry) -> Option<String> {
    match entry.path().to_str() {
        Some(file_path) => Some(file_path.to_owned()),
        None => None 
    }
}

fn is_valid_file(entry: &fs::DirEntry) -> bool {
    match entry.file_type() {
        Ok(ft) if ft.is_file() => true,
        Err(e) => {
            if let Some(file_path) = extract_filepath(entry) {
                warn!("Could not determine file type of file '{}'", file_path);
            } else {
                warn!("Could not determine file type of file.");
            }
            false
        },
        _ => false
    }
}