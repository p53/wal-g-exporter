use prometheus::{
    core::{AtomicI64, GenericGaugeVec},
    IntGaugeVec, Opts, Registry,
};

use super::Metric;
use crate::{exporter::postgres_exporter::ArchiverInfo, walg::BackupDetail};

pub struct BackupList {
    gauge: GenericGaugeVec<AtomicI64>,
}

impl BackupList {
    pub fn new(r: &Registry) -> BackupList {
        let gauge_opts = Opts::new("basebackup_list", "list of basebackups");
        let labels = ["start_wal_segment", "start_lsn"];
        let gauge: GenericGaugeVec<AtomicI64> = IntGaugeVec::new(gauge_opts, &labels).unwrap();
        r.register(Box::new(gauge.clone())).unwrap();
        return BackupList { gauge };
    }
}

impl Metric for BackupList {
    fn calculate(&self, details: &Vec<BackupDetail>, _: &ArchiverInfo) {
        for detail in details {
            self.gauge
                .with_label_values(&[&detail.wal_file_name, &detail.start_lsn.to_string()])
                .set(detail.time.timestamp());
        }
    }
}
