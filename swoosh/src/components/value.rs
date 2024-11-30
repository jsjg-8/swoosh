use std::{ fmt::Display, path::PathBuf };
use serde::{ Serialize, Deserialize };
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Clone, Debug, Default)]
pub struct ImageInfo {
    pub path: PathBuf,
    pub filename: String,
    pub size: String,
    pub status: ImageStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ImageStatus {
    #[default]
    Queued,
    Converting,
    Completed,
    Error(String),
}

impl Display for ImageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageStatus::Queued => write!(f, "Queued"),
            ImageStatus::Converting => write!(f, "Converting"),
            ImageStatus::Completed => write!(f, "Completed"),
            ImageStatus::Error(e) => write!(f, "Error: {}", e),
        }
    }
}
impl ImageStatus {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let variants = [ImageStatus::Queued, ImageStatus::Converting, ImageStatus::Completed];
        variants.choose(&mut rng).unwrap().clone()
    }
}

impl ImageInfo {
    pub fn new(path: PathBuf) -> Result<Self, String> {
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let size = std::fs
            ::metadata(&path)
            .map_err(|e| e.to_string())?
            .len();
        let size = bytesize::ByteSize(size).to_string();

        // give a random status
        let status = ImageStatus::random();

        Ok(Self {
            path,
            filename,
            size,
            status,
        })
    }
}
