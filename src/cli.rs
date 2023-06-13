use crate::exporter::Target;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Ipoth P. <pavol.ipoth@protonmail.com>")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Exports wal-g data in prometheus format", long_about = None)]
#[command(name = "wal-g-exporter")]
pub struct Cli {
    #[arg(short, long, value_enum)]
    pub target: Target,
    #[arg(default_value = "localhost", long, env)]
    pub db_host: String,
    #[arg(default_value = "5432", long, env)]
    pub db_port: String,
    #[arg(long, env)]
    pub db_user: String,
    #[arg(long, env)]
    pub db_password: String,
    #[arg(long, env)]
    pub db_name: String,
    #[arg(long, env)]
    pub db_data_dir: String,
    #[arg(long, env)]
    pub aws_region: String,
    #[arg(long, env)]
    pub aws_s3_force_path_style: String,
    #[arg(long, env)]
    pub aws_endpoint: String,
    #[arg(long, env)]
    pub aws_access_key_id: String,
    #[arg(long, env)]
    pub aws_secret_access_key: String,
    #[arg(long, env)]
    pub walg_s3_ca_cert_file: String,
    #[arg(long, env)]
    pub walg_s3_prefix: String,
    #[arg(default_value = "30", long, env)]
    pub collection_interval: u64,
    #[arg(default_value = "8080", long, env)]
    pub port: String,
}
