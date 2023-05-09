use postgres_exporter::PostgresExporter;
use prometheus::proto::MetricFamily;
use std::{collections::HashMap, sync::Arc};

pub mod postgres_exporter;

pub fn available_exporters() -> HashMap<String, Arc<dyn Exporter + Send + Sync>> {
    let mut exp: HashMap<String, Arc<dyn Exporter + Send + Sync>> = HashMap::new();
    exp.insert(String::from("pg"), Arc::new(PostgresExporter::new()));
    return exp;
}

pub trait Exporter {
    fn collect(&self) -> Vec<MetricFamily>;
}
