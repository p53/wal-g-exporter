fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    wal_g_exporter::run();
}
