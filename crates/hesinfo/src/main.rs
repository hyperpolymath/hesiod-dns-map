// SPDX-License-Identifier: PMPL-1.0-or-later
//! hesinfo: CLI for Hesiod DNS naming system.
//!
//! Subcommands:
//!   lookup   - Query a Hesiod DNS record
//!   serve    - Start the DNS + HTTP server
//!   generate - Generate a BIND-format zone file
//!   validate - Validate a zone file

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use hesiod_lib::config::HesiodConfig;
use hesiod_lib::records::MapType;
use hesiod_lib::server::run_dns_server;
use hesiod_lib::zone::HesiodZone;

#[derive(Parser)]
#[command(name = "hesinfo", version, about = "Hesiod DNS naming system CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Look up a Hesiod record via DNS query
    Lookup {
        /// Record key (e.g. username, service name)
        key: String,
        /// Map type: passwd, group, service, filsys
        map: String,
        /// DNS server address
        #[arg(long, default_value = "localhost")]
        server: String,
        /// DNS server port
        #[arg(long, default_value_t = 5353)]
        port: u16,
    },
    /// Start the Hesiod DNS server
    Serve {
        /// Path to JSON config file (from `nickel export`)
        #[arg(long)]
        config: PathBuf,
        /// UDP port for DNS
        #[arg(long, default_value_t = 53)]
        dns_port: u16,
        /// TCP port for HTTP health/metrics
        #[arg(long, default_value_t = 8080)]
        http_port: u16,
    },
    /// Generate a BIND-format zone file from config
    Generate {
        /// Path to JSON config file
        #[arg(long)]
        config: PathBuf,
        /// Output file path
        #[arg(long)]
        output: PathBuf,
    },
    /// Validate a zone file
    Validate {
        /// Path to zone file
        file: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Lookup {
            key,
            map,
            server,
            port,
        } => cmd_lookup(&key, &map, &server, port).await,
        Commands::Serve {
            config,
            dns_port,
            http_port,
        } => cmd_serve(&config, dns_port, http_port).await,
        Commands::Generate { config, output } => cmd_generate(&config, &output),
        Commands::Validate { file } => cmd_validate(&file),
    }
}

/// Send a DNS query to a Hesiod server and print the result.
async fn cmd_lookup(key: &str, map: &str, server: &str, port: u16) -> Result<()> {
    use hickory_proto::op::{Message, MessageType, OpCode, Query};
    use hickory_proto::rr::record_data::RData;
    use hickory_proto::rr::{DNSClass, Name, RecordType};
    use tokio::net::UdpSocket;

    let map_type: MapType = map.parse()?;

    // Build the query name: <key>.<map>.ns.<server-inferred-domain>
    // For simplicity, we construct the full name and let the server resolve it.
    // The user is expected to provide the full domain or we use a reasonable default.
    let qname = format!("{}.{}.ns", key, map_type.label());
    let name: Name = qname.parse().context("invalid DNS name")?;

    let mut query = Query::new();
    query.set_name(name.clone());
    query.set_query_type(RecordType::TXT);
    query.set_query_class(DNSClass::HS);

    let mut msg = Message::new();
    msg.set_id(rand_id());
    msg.set_message_type(MessageType::Query);
    msg.set_op_code(OpCode::Query);
    msg.set_recursion_desired(false);
    msg.add_query(query);

    let wire = msg.to_vec()?;

    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let addr = format!("{}:{}", server, port);
    sock.send_to(&wire, &addr).await?;

    let mut buf = vec![0u8; 4096];
    let (len, _) = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        sock.recv_from(&mut buf),
    )
    .await
    .context("DNS query timed out")??;

    let response = Message::from_vec(&buf[..len])?;

    if response.answers().is_empty() {
        println!("No records found for {}.{}", key, map_type.label());
    } else {
        for answer in response.answers() {
            let rdata: &RData = answer.data();
            if let RData::TXT(txt) = rdata {
                for s in txt.iter() {
                    println!("{}", std::str::from_utf8(s).unwrap_or("<binary>"));
                }
            }
        }
    }

    Ok(())
}

/// Start the DNS server and HTTP health endpoints.
async fn cmd_serve(config_path: &std::path::Path, dns_port: u16, http_port: u16) -> Result<()> {
    let config = HesiodConfig::from_file(config_path)?;
    let zone = HesiodZone::from_config(&config)?;

    tracing::info!(
        "loaded {} records for domain {}",
        zone.record_count(),
        zone.domain
    );

    let state = run_dns_server(zone, dns_port).await?;
    hesiod_lib::health::run_health_server(state, http_port).await?;

    Ok(())
}

/// Generate a BIND-format zone file from JSON config.
fn cmd_generate(config_path: &std::path::Path, output: &std::path::Path) -> Result<()> {
    let config = HesiodConfig::from_file(config_path)?;
    let zone = HesiodZone::from_config(&config)?;
    let bind_zone = zone.to_bind_zone();

    std::fs::write(output, &bind_zone)
        .with_context(|| format!("writing zone file to {}", output.display()))?;

    println!(
        "Generated zone file with {} records -> {}",
        zone.record_count(),
        output.display()
    );
    Ok(())
}

/// Validate a zone file by parsing each TXT record line.
fn cmd_validate(file: &std::path::Path) -> Result<()> {
    let content =
        std::fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;

    let mut errors = 0;
    let mut records = 0;

    for (line_no, line) in content.lines().enumerate() {
        let line = line.trim();
        // Skip comments and empty lines
        if line.is_empty() || line.starts_with(';') || line.starts_with('$') || line.starts_with('@') {
            continue;
        }
        // Look for HS TXT lines
        if !line.contains("HS TXT") {
            continue;
        }
        records += 1;

        // Extract the TXT data between quotes
        let txt_start = match line.find('"') {
            Some(pos) => pos + 1,
            None => {
                eprintln!("line {}: missing opening quote", line_no + 1);
                errors += 1;
                continue;
            }
        };
        let txt_end = match line.rfind('"') {
            Some(pos) if pos > txt_start => pos,
            _ => {
                eprintln!("line {}: missing closing quote", line_no + 1);
                errors += 1;
                continue;
            }
        };
        let txt_data = &line[txt_start..txt_end];

        // Determine map type from the name portion
        let name_part = line.split_whitespace().next().unwrap_or("");
        let map_type = if name_part.contains(".passwd") {
            Some(MapType::Passwd)
        } else if name_part.contains(".group") {
            Some(MapType::Group)
        } else if name_part.contains(".service") {
            Some(MapType::Service)
        } else if name_part.contains(".filsys") {
            Some(MapType::Filsys)
        } else {
            None
        };

        if let Some(mt) = map_type
            && let Err(e) = hesiod_lib::records::HesiodRecord::from_txt(mt, txt_data)
        {
            eprintln!("line {}: invalid {} record: {}", line_no + 1, mt.label(), e);
            errors += 1;
        }
    }

    if errors == 0 {
        println!("Valid: {} records checked, no errors", records);
    } else {
        println!("{} errors in {} records", errors, records);
        std::process::exit(1);
    }

    Ok(())
}

/// Generate a simple random query ID.
fn rand_id() -> u16 {
    use std::time::SystemTime;
    let t = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (t & 0xFFFF) as u16
}
