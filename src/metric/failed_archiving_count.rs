use prometheus::{
    core::{AtomicI64, GenericGauge},
    IntGauge, Opts, Registry,
};

use super::Metric;
use crate::metrics::postgres_metrics::PostgresMetricsData;

pub struct FailedArchivingCount {
    gauge: GenericGauge<AtomicI64>,
}

impl FailedArchivingCount {
    pub fn new(r: &Registry) -> FailedArchivingCount {
        let gauge_opts = Opts::new(
            "failed_archiving_count",
            "number of wal files which failed to be archived",
        );
        let gauge: GenericGauge<AtomicI64> = IntGauge::with_opts(gauge_opts).unwrap();
        r.register(Box::new(gauge.clone())).unwrap();
        return FailedArchivingCount { gauge };
    }
}

impl Metric<PostgresMetricsData> for FailedArchivingCount {
    fn calculate(&self, data: &PostgresMetricsData) {
        let mut num_failed = 0;
        let paths = &data.paths;
        for path in paths {
            if path.ends_with(".ready") {
                num_failed += num_failed;
            }
        }
        self.gauge.set(num_failed);
    }
}
