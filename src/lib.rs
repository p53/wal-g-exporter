use crate::cli::Cli;
use crate::exporter::available_exporters;

use clap::Parser;
use log::{error, info};
use prometheus::proto::MetricFamily;
use prometheus::{Encoder, TextEncoder};
use std::process::exit;
use std::sync::{mpsc, Arc};
use std::{thread, time, vec};
use tiny_http::{Response, Server};

pub mod cli;
pub mod exporter;
pub mod metric;
pub mod walg;

pub fn run() {
    info!("Starting application");
    let args = Cli::parse();
    let mut result_metrics: Vec<MetricFamily> = vec![];
    let (tx, rx) = mpsc::channel();

    let listen_addr = format!("0.0.0.0:{}", args.port);
    let server = Server::http(listen_addr).unwrap();

    let exporters = available_exporters(
        args.db_host,
        args.db_port,
        args.db_user,
        args.db_password,
        args.db_name,
    );

    if let Some(ex) = exporters.get(&args.target) {
        let exp = Arc::clone(ex);
        thread::spawn(move || loop {
            let val = exp.collect();
            tx.send(val).unwrap();

            let collect_period = time::Duration::from_secs(args.collection_interval);
            thread::sleep(collect_period);
        });
    } else {
        error!("Invalid exporter type");
        exit(1);
    }

    for request in server.incoming_requests() {
        let mut buffer = vec![];

        if let Ok(val) = rx.try_recv() {
            if val.len() > 0 {
                result_metrics = val;
            }
        }

        if result_metrics.len() > 0 {
            let encoder = TextEncoder::new();
            encoder.encode(&result_metrics, &mut buffer).unwrap();
        }

        let response = Response::from_data(buffer);
        request.respond(response).unwrap();
    }

    info!("Application stopped");
}
