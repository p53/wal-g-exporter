use prometheus::{
    core::{AtomicI64, GenericGauge},
    IntGauge, Opts, Registry,
};

use super::Metric;
use crate::walg::BackupDetail;

pub struct OldestBackup {
    gauge: GenericGauge<AtomicI64>,
}

impl OldestBackup {
    pub fn new(r: &Registry) -> OldestBackup {
        let gauge_opts = Opts::new("oldest_basebackup", "timestamp of oldest basebackup");
        let gauge: GenericGauge<AtomicI64> = IntGauge::with_opts(gauge_opts).unwrap();
        r.register(Box::new(gauge.clone())).unwrap();
        return OldestBackup { gauge };
    }
}

impl Metric for OldestBackup {
    fn calculate(&self, details: &Vec<BackupDetail>) {
        if let Some(detail) = details.first() {
            self.gauge.set(detail.time.timestamp());
        }
    }
}
