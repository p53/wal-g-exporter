use std::{sync::Arc, vec};

use crate::walg::BackupDetail;
use backup_count::BackupCount;
use backup_list::BackupList;
use last_backup::LastBackup;
use last_backup_duration::LastBackupDuration;
use last_backup_size::LastBackupSizeCompressed;
use last_backup_size::LastBackupSizeUnCompressed;
use oldest_backup::OldestBackup;
use prometheus::proto::MetricFamily;
use prometheus::Registry;

pub mod backup_count;
pub mod backup_list;
pub mod last_backup;
pub mod last_backup_duration;
pub mod last_backup_size;
pub mod oldest_backup;

pub trait Metric {
    fn calculate(&self, details: &Vec<BackupDetail>);
}

pub struct Metrics {
    pub list: Vec<Arc<dyn Metric + Send + Sync>>,
    pub registry: Registry,
}

impl Metrics {
    pub fn new(r: Registry) -> Metrics {
        return Metrics {
            list: vec![
                Arc::new(BackupCount::new(&r)),
                Arc::new(LastBackup::new(&r)),
                Arc::new(OldestBackup::new(&r)),
                Arc::new(LastBackupDuration::new(&r)),
                Arc::new(LastBackupSizeCompressed::new(&r)),
                Arc::new(LastBackupSizeUnCompressed::new(&r)),
                Arc::new(BackupList::new(&r)),
            ],
            registry: r,
        };
    }

    pub fn gather(&self, details: Vec<BackupDetail>) -> Vec<MetricFamily> {
        let mtr_list = &self.list;
        let det_ref = &details;
        for metr in mtr_list {
            metr.calculate(det_ref);
        }
        self.registry.gather()
    }
}
