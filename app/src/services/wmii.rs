use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use wmi::WMIConnection;

#[derive(Debug, Deserialize, Clone)]
pub struct WmiDisk {
    #[serde(rename = "DeviceID")]
    pub device_id: String,
    #[serde(rename = "Model")]
    pub model: Option<String>,
    #[serde(rename = "MediaType")]
    pub media_type: Option<String>,
    #[serde(rename = "InterfaceType")]
    pub interface_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LogicalDiskToPartition {
    #[serde(rename = "Antecedent")]
    antecedent: String,
    #[serde(rename = "Dependent")]
    dependent: String,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    refreshed_on: Instant,
    disks: Vec<WmiDisk>,
    logical_map: HashMap<String, WmiDisk>,
}

pub struct WmiService {
    cache: Arc<RwLock<CacheEntry>>,
    cache_ttl: Duration,
}

impl WmiService {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(CacheEntry {
                refreshed_on: Instant::now() - Duration::from_secs(3600),
                disks: Vec::new(),
                logical_map: HashMap::new(),
            })),
            cache_ttl: Duration::from_secs(3600),
        }
    }

    fn fetch_disks() -> Result<Vec<WmiDisk>> {
        let connection = WMIConnection::new()?;
        let disks: Vec<WmiDisk> = connection.raw_query(
            "SELECT DeviceID, Model, MediaType, InterfaceType FROM Win32_DiskDrive"
        )?;
        Ok(disks)
    }

    fn fetch_logical_map(disks: &[WmiDisk]) -> Result<HashMap<String, WmiDisk>> {
        let connection = WMIConnection::new()?;
        let links: Vec<LogicalDiskToPartition> = connection.raw_query(
            "SELECT Antecedent, Dependent FROM Win32_LogicalDiskToPartition"
        )?;

        let mut logical_map = HashMap::new();

        for link in links {
            if let Some(disk_id) = link.antecedent.split("Disk #").nth(1) {
                if let Some(index_str) = disk_id.split(',').next() {
                    if let Ok(idx) = index_str.parse::<u32>() {
                        if let Some(drive_letter) = link
                            .dependent
                            .split("DeviceID=")
                            .nth(1)
                            .map(|s| s.trim_matches('"'))
                        {
                            if let Some(disk) = disks.iter().find(|d| d.device_id.ends_with(&idx.to_string())) {
                                logical_map.insert(drive_letter.to_string(), disk.clone());
                            }
                        }
                    }
                }
            }
        }

        Ok(logical_map)
    }

    pub fn get_disks(&self) -> Result<Vec<WmiDisk>> {
        let mut cache = self.cache.write().unwrap();
        if cache.refreshed_on.elapsed() >= self.cache_ttl {
            let disks = Self::fetch_disks()?;
            cache.disks = disks.clone();
            cache.refreshed_on = Instant::now();
        }
        Ok(cache.disks.clone())
    }

    pub fn map_logical_to_physical(&self) -> Result<HashMap<String, WmiDisk>> {
        let mut cache = self.cache.write().unwrap();
        if cache.refreshed_on.elapsed() >= self.cache_ttl || cache.logical_map.is_empty() {
            let disks = Self::fetch_disks()?;
            let logical_map = Self::fetch_logical_map(&disks)?;
            cache.disks = disks;
            cache.logical_map = logical_map;
            cache.refreshed_on = Instant::now();
        }
        Ok(cache.logical_map.clone())
    }
}
