use clap::ValueEnum;
use postgres_exporter::PostgresExporter;
use prometheus::proto::MetricFamily;
use std::{collections::HashMap, sync::Arc};

pub mod postgres_exporter;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Hash, Debug)]
pub enum Target {
    Postgres,
}

pub fn available_exporters(
    host: String,
    port: String,
    user: String,
    password: String,
    db_name: String,
    db_data_dir: String,
) -> HashMap<Target, Arc<dyn Exporter + Send + Sync>> {
    let mut exp: HashMap<Target, Arc<dyn Exporter + Send + Sync>> = HashMap::new();
    exp.insert(
        Target::Postgres,
        Arc::new(PostgresExporter::new(
            host,
            port,
            user,
            password,
            db_name,
            db_data_dir,
        )),
    );
    return exp;
}

pub trait Exporter {
    fn collect(&self) -> Vec<MetricFamily>;
}
