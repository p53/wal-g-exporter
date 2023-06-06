use prometheus::{
    core::{AtomicI64, GenericGauge},
    IntGauge, Opts, Registry,
};

use super::Metric;
use crate::{walg::BackupDetail};

pub struct LastBackupDuration {
    gauge: GenericGauge<AtomicI64>,
}

impl LastBackupDuration {
    pub fn new(r: &Registry) -> LastBackupDuration {
        let gauge_opts = Opts::new("last_basebackup_duration", "duration of last basebackup");
        let gauge: GenericGauge<AtomicI64> = IntGauge::with_opts(gauge_opts).unwrap();
        r.register(Box::new(gauge.clone())).unwrap();
        return LastBackupDuration { gauge };
    }
}

impl Metric<Vec<BackupDetail>> for LastBackupDuration {
    fn calculate(&self, details: &Vec<BackupDetail>) {
        if let Some(detail) = details.last() {
            self.gauge
                .set(detail.finish_time.timestamp() - detail.start_time.timestamp());
        }
    }
}
