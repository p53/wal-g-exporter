use prometheus::{
    core::{AtomicI64, GenericGauge},
    IntGauge, Opts, Registry,
};

use super::Metric;
use crate::{exporter::postgres_exporter::ArchiverInfo, walg::BackupDetail};

pub struct LastBackupSizeCompressed {
    gauge: GenericGauge<AtomicI64>,
}

pub struct LastBackupSizeUnCompressed {
    gauge: GenericGauge<AtomicI64>,
}

impl LastBackupSizeCompressed {
    pub fn new(r: &Registry) -> LastBackupSizeCompressed {
        let gauge_opts = Opts::new(
            "last_basebackup_size_compressed",
            "compressed size of last basebackup",
        );
        let gauge: GenericGauge<AtomicI64> = IntGauge::with_opts(gauge_opts).unwrap();
        r.register(Box::new(gauge.clone())).unwrap();
        return LastBackupSizeCompressed { gauge };
    }
}

impl Metric for LastBackupSizeCompressed {
    fn calculate(&self, details: &Vec<BackupDetail>, _: &ArchiverInfo) {
        if let Some(detail) = details.last() {
            self.gauge.set(detail.compressed_size.into());
        }
    }
}

impl LastBackupSizeUnCompressed {
    pub fn new(r: &Registry) -> LastBackupSizeUnCompressed {
        let gauge_opts = Opts::new(
            "last_basebackup_size_uncompressed",
            "uncompressed size of last basebackup",
        );
        let gauge: GenericGauge<AtomicI64> = IntGauge::with_opts(gauge_opts).unwrap();
        r.register(Box::new(gauge.clone())).unwrap();
        return LastBackupSizeUnCompressed { gauge };
    }
}

impl Metric for LastBackupSizeUnCompressed {
    fn calculate(&self, details: &Vec<BackupDetail>, _: &ArchiverInfo) {
        if let Some(detail) = details.last() {
            self.gauge.set(detail.uncompressed_size.into());
        }
    }
}
