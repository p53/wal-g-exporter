use std::{sync::Arc, vec};

use crate::exporter::postgres_exporter::ArchiverInfo;
use crate::metric::incremental_count::IncrementalBackupCount;
use crate::metric::last_archived_time::LastArchivedTime;
use crate::metric::Metric;
use crate::walg::BackupDetail;
use prometheus::proto::MetricFamily;
use prometheus::Registry;

pub struct PostgresMetricsData {
    pub details: Vec<BackupDetail>,
    pub archiver_info: ArchiverInfo,
}

pub struct PostgresMetrics<PostgresMetricsData> {
    pub list: Vec<Arc<dyn Metric<PostgresMetricsData> + Send + Sync>>,
    pub registry: Registry,
}

impl<'a> PostgresMetrics<PostgresMetricsData> {
    pub fn new(r: Registry) -> PostgresMetrics<PostgresMetricsData> {
        return PostgresMetrics {
            list: vec![
                Arc::new(LastArchivedTime::new(&r)),
                Arc::new(IncrementalBackupCount::new(&r)),
            ],
            registry: r,
        };
    }

    pub fn gather(&self, data: &'a PostgresMetricsData) -> Vec<MetricFamily> {
        let mtr_list = &self.list;
        for metr in mtr_list {
            metr.calculate(&data);
        }
        self.registry.gather()
    }
}
