use prometheus::{
    core::{AtomicI64, GenericGauge},
    IntGauge, Opts, Registry,
};

use super::Metric;
use crate::{walg::BackupDetail};

pub struct LastBackup {
    gauge: GenericGauge<AtomicI64>,
}

impl LastBackup {
    pub fn new(r: &Registry) -> LastBackup {
        let gauge_opts = Opts::new("last_basebackup", "timestamp of last basebackup");
        let gauge: GenericGauge<AtomicI64> = IntGauge::with_opts(gauge_opts).unwrap();
        r.register(Box::new(gauge.clone())).unwrap();
        return LastBackup { gauge };
    }
}

impl Metric<Vec<BackupDetail>> for LastBackup {
    fn calculate(&self, details: &Vec<BackupDetail>) {
        if let Some(detail) = details.last() {
            self.gauge.set(detail.time.timestamp());
        }
    }
}
