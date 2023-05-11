use postgres_exporter::PostgresExporter;
use prometheus::proto::MetricFamily;
use std::{collections::HashMap, sync::Arc};

pub mod postgres_exporter;

pub fn available_exporters(
    host: String,
    port: String,
    user: String,
    password: String,
    db_name: String,
) -> HashMap<String, Arc<dyn Exporter + Send + Sync>> {
    let mut exp: HashMap<String, Arc<dyn Exporter + Send + Sync>> = HashMap::new();
    exp.insert(
        String::from("pg"),
        Arc::new(PostgresExporter::new(host, port, user, password, db_name)),
    );
    return exp;
}

pub trait Exporter {
    fn collect(&self) -> Vec<MetricFamily>;
}
