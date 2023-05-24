use std::{sync::Arc, vec};

use crate::exporter::postgres_exporter::ArchiverInfo;
use crate::walg::BackupDetail;
use backup_count::BackupCount;
use backup_list::BackupList;
use incremental_count::IncrementalBackupCount;
use last_archived_time::LastArchivedTime;
use last_backup::LastBackup;
use last_backup_duration::LastBackupDuration;
use last_backup_size::LastBackupSizeCompressed;
use last_backup_size::LastBackupSizeUnCompressed;
use oldest_backup::OldestBackup;
use prometheus::proto::MetricFamily;
use prometheus::Registry;

pub mod backup_count;
pub mod backup_list;
pub mod incremental_count;
pub mod last_archived_time;
pub mod last_backup;
pub mod last_backup_duration;
pub mod last_backup_size;
pub mod oldest_backup;

pub trait Metric {
    fn calculate(&self, details: &Vec<BackupDetail>, archiver_info: &ArchiverInfo);
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
                Arc::new(LastArchivedTime::new(&r)),
                Arc::new(IncrementalBackupCount::new(&r)),
            ],
            registry: r,
        };
    }

    pub fn gather(
        &self,
        details: Vec<BackupDetail>,
        archiver_info: ArchiverInfo,
    ) -> Vec<MetricFamily> {
        let mtr_list = &self.list;
        let det_ref = &details;
        for metr in mtr_list {
            metr.calculate(det_ref, &archiver_info);
        }
        self.registry.gather()
    }
}
