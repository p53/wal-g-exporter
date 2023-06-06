use chrono::{DateTime, Utc};
use prometheus::{
    core::{AtomicI64, GenericGauge},
    IntGauge, Opts, Registry,
};

use super::Metric;
use crate::{
    metrics::postgres_metrics::PostgresMetricsData,
};

const WAL_FACTOR: i64 = 256;

pub struct IncrementalBackupCount {
    gauge: GenericGauge<AtomicI64>,
}

impl IncrementalBackupCount {
    pub fn new(r: &Registry) -> IncrementalBackupCount {
        let gauge_opts = Opts::new(
            "incremental_count",
            "number of incremental backups since last basebackup",
        );
        let gauge: GenericGauge<AtomicI64> = IntGauge::with_opts(gauge_opts).unwrap();
        r.register(Box::new(gauge.clone())).unwrap();
        return IncrementalBackupCount { gauge };
    }

    fn wall_diff(&self, a: &str, b: &str) -> i64 {
        if a[0..8] != b[0..8] {
            return -1;
        }

        let a_prefix = i64::from_str_radix(&a[8..16], 16).unwrap();
        let b_prefix = i64::from_str_radix(&b[8..16], 16).unwrap();
        let a_suffix = i64::from_str_radix(&a[16..24], 16).unwrap();
        let b_suffix = i64::from_str_radix(&b[16..24], 16).unwrap();

        let a_int = a_prefix * WAL_FACTOR + a_suffix;
        let b_int = b_prefix * WAL_FACTOR + b_suffix;

        return a_int - b_int;
    }
}

impl Metric<PostgresMetricsData> for IncrementalBackupCount {
    fn calculate(&self, info: &PostgresMetricsData) {
        if info.archiver_info.last_archived_time == DateTime::<Utc>::MIN_UTC {
            self.gauge.set(0);
        }
        if info.details.len() == 0 {
            self.gauge.set(0);
        }
        self.gauge.set(self.wall_diff(
            &info.archiver_info.last_archived_wal,
            &info.details.last().unwrap().wal_file_name,
        ));
    }
}
