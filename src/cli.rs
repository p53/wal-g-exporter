use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "demo")]
pub struct Cli {
    #[arg(short, long)]
    pub target: String,
    #[arg(short, long, env)]
    pub db_password: String,
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
