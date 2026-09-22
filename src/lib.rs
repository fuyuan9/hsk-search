#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::too_many_lines
)]

pub mod app;
pub mod cjk;
pub mod models;
pub mod search;
pub mod ui;

pub static DATASET_JSON: &str = include_str!("../assets/hsk30_index.json");

/// Loads and parses the embedded HSK 3.0 dataset.
///
/// # Errors
/// Returns error if JSON deserialization fails.
pub fn load_dataset() -> Result<Vec<models::HskItem>, serde_json::Error> {
    serde_json::from_str(DATASET_JSON)
}
