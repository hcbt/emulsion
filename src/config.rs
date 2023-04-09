use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    camera: CameraConfig,
    processing: ProcessingConfig,
    x_raw_studio: XRawStudioConfig,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CameraConfig {
    model: String,
    firmware: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessingConfig {
    color_space: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct XRawStudioConfig {
    version: String,
    serial_number: String,
    simulations_path: String,
}