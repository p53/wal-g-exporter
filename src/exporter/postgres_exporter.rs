use super::Exporter;

use crate::metrics::postgres_metrics::PostgresMetricsData;
use crate::metrics::{postgres_metrics::PostgresMetrics, Metrics};
use crate::walg::BackupDetail;

use chrono::{DateTime, Utc};
use log::{error, warn};
use postgres::{Client, NoTls};
use prometheus::proto::MetricFamily;
use prometheus::Registry;
use retry::{delay::Fixed, retry_with_index, OperationResult};
use std::fs;
use std::path::PathBuf;
use std::{
    process::{exit, Command},
    sync::{Arc, Mutex},
    vec,
};

const STAT_ARCHIVER_QUERY: &str = concat!(
    "SELECT COALESCE(archived_count, 0),",
    "COALESCE(failed_count, 0),",
    "COALESCE(last_archived_wal, ''),",
    "COALESCE(last_archived_time, to_timestamp(0)),",
    "COALESCE(last_failed_wal, ''),",
    "COALESCE(last_failed_time, to_timestamp(0)) FROM pg_stat_archiver",
);

const IS_NOT_IN_RECOVERY_QUERY: &str = "SELECT NOT pg_is_in_recovery()";

pub struct ArchiverInfo {
    pub last_archived_wal: Box<String>,
    pub last_archived_time: DateTime<Utc>,
}

impl ArchiverInfo {
    pub fn new() -> ArchiverInfo {
        let a_wal = "";
        ArchiverInfo {
            last_archived_time: DateTime::<Utc>::MIN_UTC,
            last_archived_wal: Box::new(a_wal.to_string()),
        }
    }
}
pub struct PostgresExporter {
    metrics: Arc<Mutex<Metrics<Vec<BackupDetail>>>>,
    pg_metrics: Arc<Mutex<PostgresMetrics<PostgresMetricsData>>>,
    client: Arc<Mutex<Client>>,
    db_data_dir: Arc<Mutex<String>>,
}

impl<'a> PostgresExporter {
    pub fn new(
        host: String,
        port: String,
        user: String,
        password: String,
        db_name: String,
        db_data_dir: String,
    ) -> Self {
        let general_registry = Registry::new();
        let pg_registry = Registry::new();
        let metrics = Metrics::new(general_registry);
        let pg_metrics = PostgresMetrics::new(pg_registry);

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
                return Self {
                    metrics: Arc::new(Mutex::new(metrics)),
                    pg_metrics: Arc::new(Mutex::new(pg_metrics)),
                    client: Arc::new(Mutex::new(c)),
                    db_data_dir: Arc::new(Mutex::new(db_data_dir)),
                };
            }
            Err(e) => {
                error!("{}", e);
                exit(1)
            }
        }
    }

    fn is_master(&self) -> bool {
        for row in self
            .client
            .lock()
            .unwrap()
            .query(IS_NOT_IN_RECOVERY_QUERY, &[])
            .unwrap_or_else(|e| {
                error!("{}", e);
                exit(1);
            })
        {
            return row.get(0);
        }
        return false;
    }
}

impl<'a> Exporter for PostgresExporter {
    fn collect(&self) -> Vec<MetricFamily> {
        if self.is_master() {
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

            let data_dir_path = self.db_data_dir.lock().unwrap();
            let dir_iterator =
                fs::read_dir::<String>(data_dir_path.to_string()).unwrap_or_else(|e| {
                    error!("{}", e);
                    exit(1);
                });
            let mut paths: Vec<PathBuf> = vec![];

            for entry in dir_iterator {
                paths.push(entry.unwrap().path());
            }

            let mut archiver_info = ArchiverInfo::new();

            for row in self
                .client
                .lock()
                .unwrap()
                .query(STAT_ARCHIVER_QUERY, &[])
                .unwrap_or_else(|e| {
                    error!("{}", e);
                    exit(1);
                })
            {
                archiver_info.last_archived_time = row.get(3);
                let a_wal: String = row.get(2);
                archiver_info.last_archived_wal = Box::new(a_wal);
            }

            if output.stdout.len() != 0 {
                let deserialized: Vec<BackupDetail> =
                    serde_json::from_str(std::str::from_utf8(&output.stdout).unwrap())
                        .unwrap_or_else(|e| {
                            error!("{}", e);
                            exit(1)
                        });

                let general_data = self.metrics.lock().unwrap().gather(&deserialized);

                let data = PostgresMetricsData {
                    details: deserialized,
                    archiver_info,
                    paths,
                };

                let pg_data = self.pg_metrics.lock().unwrap().gather(&data);

                general_data
                    .iter()
                    .cloned()
                    .chain(pg_data.iter().cloned())
                    .collect::<Vec<_>>()
            } else {
                warn!("{}", std::str::from_utf8(&output.stderr).unwrap());
                let result: Vec<MetricFamily> = vec![];
                return result;
            }
        } else {
            warn!("{}", "node not master, not collecting metrics");
            let result: Vec<MetricFamily> = vec![];
            return result;
        }

        // look on fs for .ready files
    }
}
