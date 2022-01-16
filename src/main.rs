use clap::Parser;
use env_logger::{
    Builder,
    Env,
};
use log::{debug,info};
use prometheus_exporter::prometheus::{IntGauge, labels, opts, register_int_gauge};
use serde_json::Result;
use std::convert::TryInto;
use std::net::SocketAddr;
use std::str;

mod borg;


// Useful documentation:
// - chrono: https://docs.rs/chrono/latest/chrono/
// - clap: https://docs.rs/clap/latest/clap/
// - prometheus: https://docs.rs/prometheus/latest/prometheus/
// - prometheus-exporter: https://docs.rs/prometheus_exporter/latest/prometheus_exporter/

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// IP to listen to
    #[clap(long, default_value_t = String::from("0.0.0.0"))]
    listen_ip: String,

    /// Port to listen to
    #[clap(long, default_value_t = 9884)]
    listen_port: u16,

    /// Borg repository to monitor
    #[clap(long)]
    repository: String,
}


struct BorgMetrics {
    archives_latest_time: IntGauge,
    archives_total: IntGauge,
    chunks_total: IntGauge,
    archives_compressed_size: IntGauge,
    archives_size: IntGauge,
    chunks_deduplicated_total: IntGauge,
    archives_deduplicated_compressed_size: IntGauge,
    archives_deduplicated_size: IntGauge,
}

impl BorgMetrics {
    fn new(repository: &str) -> BorgMetrics {
        let labels = labels!{"repository" => repository};

        BorgMetrics {
            archives_latest_time: register_int_gauge!(opts!("borg_archives_latest_time", "Timestamp of the latest archive", labels))
                .expect("Unable to create gauge borg_archives_latest_time"),
            archives_total: register_int_gauge!(opts!("borg_archives_total", "Number of available archives", labels))
                .expect("Unable to create gauge borg_archives_total"),
            chunks_total: register_int_gauge!(opts!("borg_chunks_total", "Number of chunks", labels))
                .expect("Unable to create gauge borg_chunks_total"),
            archives_compressed_size: register_int_gauge!(opts!("borg_archives_compressed_size", "Compressed size of all archives", labels))
                .expect("Unable to create gauge borg_archives_compressed_size"),
            archives_size: register_int_gauge!(opts!("borg_archives_size", "Original size of all archives", labels))
                .expect("Unable to create gauge borg_archives_size"),
            chunks_deduplicated_total: register_int_gauge!(opts!("borg_chunks_deduplicated_total", "Number of unique chunks", labels))
                .expect("Unable to create gauge borg_chunks_deduplicated_total"),
            archives_deduplicated_compressed_size: register_int_gauge!(opts!("borg_archives_deduplicated_compressed_size", "Compressed, deduplicated size of all archives", labels))
                .expect("Unable to create gauge borg_archives_deduplicated_compressed_size"),
            archives_deduplicated_size: register_int_gauge!(opts!("borg_archives_deduplicated_size", "Deduplicated size of all archives", labels))
                .expect("Unable to create gauge borg_archives_deduplicated_size"),
        }
    }
}


fn main() -> Result<()> {
    let args = Args::parse();

    Builder::from_env(Env::default().default_filter_or("info")).init();

    let addr: SocketAddr = format!("{}:{}", args.listen_ip, args.listen_port).parse().expect("Unable to parse listen address");
    let exporter = prometheus_exporter::start(addr).expect("Unable to start exporter");
    let metrics = BorgMetrics::new(&args.repository);

    loop {
        info!("Waiting for a request...");
        let _guard = exporter.wait_request();
        info!("Received request, updating metrics...");

        let archives_list = borg::ArchivesList::retrieve(&args.repository)?;
        debug!("Borg output: {:?}", archives_list);

        let last_archive = archives_list.last_archive();
        metrics.archives_latest_time.set(last_archive.time.timestamp());
        metrics.archives_total.set(archives_list.archives.len().try_into().unwrap());

        let borg_info = borg::Info::retrieve(&args.repository)?;
        metrics.chunks_total.set(borg_info.cache.stats.total_chunks.try_into().unwrap());
        metrics.archives_compressed_size.set(borg_info.cache.stats.total_csize.try_into().unwrap());
        metrics.archives_size.set(borg_info.cache.stats.total_size.try_into().unwrap());
        metrics.chunks_deduplicated_total.set(borg_info.cache.stats.total_unique_chunks.try_into().unwrap());
        metrics.archives_deduplicated_compressed_size.set(borg_info.cache.stats.unique_csize.try_into().unwrap());
        metrics.archives_deduplicated_size.set(borg_info.cache.stats.unique_size.try_into().unwrap());
    }
}
