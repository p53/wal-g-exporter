use prometheus::{
    core::{AtomicI64, GenericGauge},
    IntGauge, Opts, Registry,
};

use super::Metric;
use crate::walg::BackupDetail;

pub struct BackupCount {
    gauge: GenericGauge<AtomicI64>,
}

impl BackupCount {
    pub fn new(r: &Registry) -> BackupCount {
        let gauge_opts = Opts::new("basebackup_count", "number of basebackups");
        let gauge: GenericGauge<AtomicI64> = IntGauge::with_opts(gauge_opts).unwrap();
        r.register(Box::new(gauge.clone())).unwrap();
        return BackupCount { gauge };
    }
}

impl Metric for BackupCount {
    fn calculate(&self, details: &Vec<BackupDetail>) {
        self.gauge.set(details.len().try_into().unwrap())
    }
}
