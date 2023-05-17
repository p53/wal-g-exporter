use prometheus::{
    core::{AtomicI64, GenericGauge},
    IntGauge, Opts, Registry,
};

use super::Metric;
use crate::{exporter::postgres_exporter::ArchiverInfo, walg::BackupDetail};

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

impl Metric for LastArchivedTime {
    fn calculate(&self, _: &Vec<BackupDetail>, archiver_info: &ArchiverInfo) {
        self.gauge.set(archiver_info.last_archived_time.timestamp());
    }
}
