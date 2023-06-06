use prometheus::{
    core::{AtomicI64, GenericGauge},
    IntGauge, Opts, Registry,
};

use super::Metric;
use crate::{
    metrics::postgres_metrics::PostgresMetricsData,
};

pub struct LastArchivedTime {
    gauge: GenericGauge<AtomicI64>,
}

impl LastArchivedTime {
    pub fn new(r: &Registry) -> LastArchivedTime {
        let gauge_opts = Opts::new(
            "last_archived_time",
            "timestamp of upload of last wal archive",
        );
        let gauge: GenericGauge<AtomicI64> = IntGauge::with_opts(gauge_opts).unwrap();
        r.register(Box::new(gauge.clone())).unwrap();
        return LastArchivedTime { gauge };
    }
}

impl Metric<PostgresMetricsData> for LastArchivedTime {
    fn calculate(&self, data: &PostgresMetricsData) {
        self.gauge
            .set(data.archiver_info.last_archived_time.timestamp());
    }
}
