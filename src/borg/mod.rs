use chrono::DateTime;
use chrono::offset::Utc;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::process::Command;
use std::str;

mod datetime_format;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArchivesList {
    pub archives: Vec<Archive>,
    encryption: Encryption,
    repository: Repository,
}

impl ArchivesList {
    pub fn retrieve(borg_repository: &str) -> Result<ArchivesList> {
        let command_result = Command::new("borg")
            .arg("list")
            .arg("--json")
            .arg(borg_repository)
            .output()
            .expect("Failed to execute borg");

        let stdout = str::from_utf8(&command_result.stdout).unwrap();
        debug!("Stdout: {:?}", stdout);

        serde_json::from_str(stdout)
    }

    pub fn last_archive(&self) -> &Archive {
        self.archives.iter()
            .max_by(|a, b| a.time.cmp(&b.time) )
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub cache: Cache,
    encryption: Encryption,
    repository: Repository,
    security_dir: String,
}

impl Info {
    pub fn retrieve(borg_repository: &str) -> Result<Info> {
        let command_result = Command::new("borg")
            .arg("info")
            .arg("--json")
            .arg("--remote-path=borg1")
            .arg(borg_repository)
            .output()
            .expect("Failed to execute borg");

        let stdout = str::from_utf8(&command_result.stdout).unwrap();
        debug!("Stdout: {:?}", stdout);

        serde_json::from_str(stdout)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    path: String,
    pub stats: CacheStats,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheStats {
    pub total_chunks: u64,
    pub total_csize: u64,
    pub total_size: u64,
    pub total_unique_chunks: u64,
    pub unique_csize: u64,
    pub unique_size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Archive {
    pub archive: String,
    pub barchive: String,
    pub id: String,
    pub name: String,

    #[serde(with = "datetime_format")]
    pub start: DateTime<Utc>,

    #[serde(with = "datetime_format")]
    pub time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Encryption{
    mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    id: String,
    #[serde(with = "datetime_format")]
    last_modified: DateTime<Utc>,
    location: String,
}
