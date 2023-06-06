use std::{sync::Arc, vec};

use crate::metric::backup_count::BackupCount;
use crate::metric::backup_list::BackupList;
use crate::metric::last_backup::LastBackup;
use crate::metric::last_backup_duration::LastBackupDuration;
use crate::metric::last_backup_size::LastBackupSizeCompressed;
use crate::metric::last_backup_size::LastBackupSizeUnCompressed;
use crate::metric::oldest_backup::OldestBackup;
use crate::metric::Metric;
use crate::walg::BackupDetail;
use prometheus::proto::MetricFamily;
use prometheus::Registry;

pub mod postgres_metrics;

pub struct Metrics<BackupDetail> {
    pub list: Vec<Arc<dyn Metric<BackupDetail> + Send + Sync>>,
    pub registry: Registry,
}

impl Metrics<Vec<BackupDetail>> {
    pub fn new(r: Registry) -> Metrics<Vec<BackupDetail>> {
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

    pub fn gather(&self, details: &Vec<BackupDetail>) -> Vec<MetricFamily> {
        let mtr_list = &self.list;
        for metr in mtr_list {
            metr.calculate(details);
        }
        self.registry.gather()
    }
}
