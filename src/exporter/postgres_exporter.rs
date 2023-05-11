use super::Exporter;

use crate::metric::Metrics;
use crate::walg::BackupDetail;

use log::{error, warn};
use postgres::{Client, NoTls};
use prometheus::proto::MetricFamily;
use prometheus::Registry;
use retry::{delay::Fixed, retry_with_index, OperationResult};
use std::{
    process::{exit, Command},
    sync::{Arc, Mutex},
    vec,
};

pub struct PostgresExporter {
    metrics: Arc<Mutex<Metrics>>,
    client: Arc<Mutex<Client>>,
}

impl PostgresExporter {
    pub fn new(
        host: String,
        port: String,
        user: String,
        password: String,
        db_name: String,
    ) -> PostgresExporter {
        let r = Registry::new();
        let metrics = Metrics::new(r);
        let connect_string = format!(
            "host={} port={} user={} password={} dbname={}",
            host, port, user, password, db_name
        );

        let client = retry_with_index(Fixed::from_millis(2000), |current_try| {
            if current_try > 10 {
                return OperationResult::Err("did not succeed within 10 tries");
            }

            let client_local = Client::connect(&connect_string, NoTls);
            match client_local {
                Ok(c) => OperationResult::Ok(c),
                Err(e) => {
                    warn!("{}", e);
                    OperationResult::Retry("retry")
                }
            }
        });

        match client {
            Ok(c) => {
                return PostgresExporter {
                    metrics: Arc::new(Mutex::new(metrics)),
                    client: Arc::new(Mutex::new(c)),
                };
            }
            Err(e) => {
                error!("{}", e);
                exit(1)
            }
        }
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
