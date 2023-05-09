use super::Exporter;

use crate::metric::Metrics;
use crate::walg::BackupDetail;

use log::{error, warn};
use prometheus::proto::MetricFamily;
use prometheus::Registry;
use std::{
    process::{exit, Command},
    sync::{Arc, Mutex},
    vec,
};

pub struct PostgresExporter {
    metrics: Arc<Mutex<Metrics>>,
}

impl PostgresExporter {
    pub fn new() -> PostgresExporter {
        let r = Registry::new();
        let metrics = Metrics::new(r);
        return PostgresExporter {
            metrics: Arc::new(Mutex::new(metrics)),
        };
    }
}

impl Exporter for PostgresExporter {
    fn collect(&self) -> Vec<MetricFamily> {
        let output = Command::new("wal-g-pg")
            .arg("backup-list")
            .arg("--json")
            .arg("--detail")
            .output()
            .expect("process failed to execute");

        if !output.status.success() {
            error!("{}", std::str::from_utf8(&output.stderr).unwrap());
            exit(1)
        }

        if output.stdout.len() != 0 {
            let deserialized: Vec<BackupDetail> = serde_json::from_str(
                std::str::from_utf8(&output.stdout).unwrap(),
            )
            .unwrap_or_else(|e| {
                error!("{}", e);
                exit(1)
            });

            self.metrics.lock().unwrap().gather(deserialized)
        } else {
            warn!("{}", std::str::from_utf8(&output.stderr).unwrap());
            let result: Vec<MetricFamily> = vec![];
            return result;
        }

        // connect to db to list pg archives
        // look on fs for .ready files
    }
}
