// SPDX-License-Identifier: PMPL-1.0-or-later
//! UDP DNS server handling HS-class TXT queries using hickory-proto.

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use hickory_proto::op::{Header, Message, OpCode, ResponseCode};
use hickory_proto::rr::rdata::TXT;
use hickory_proto::rr::record_data::RData;
use hickory_proto::rr::{DNSClass, Name, Record, RecordType};
use tokio::net::UdpSocket;
use tracing::{debug, error, info, warn};

use crate::records::MapType;
use crate::zone::HesiodZone;

/// DNS class value for Hesiod (HS = 4).
const DNS_CLASS_HS: u16 = 4;

/// Shared server state.
pub struct DnsServerState {
    pub zone: HesiodZone,
    pub query_count: std::sync::atomic::AtomicU64,
    pub start_time: std::time::Instant,
}

/// Run the Hesiod DNS server on the given port.
pub async fn run_dns_server(zone: HesiodZone, port: u16) -> Result<Arc<DnsServerState>> {
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
    let socket = UdpSocket::bind(addr)
        .await
        .with_context(|| format!("binding UDP socket on port {}", port))?;

    info!("Hesiod DNS server listening on {}", addr);

    let state = Arc::new(DnsServerState {
        zone,
        query_count: std::sync::atomic::AtomicU64::new(0),
        start_time: std::time::Instant::now(),
    });

    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        let mut buf = vec![0u8; 4096];
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, src)) => {
                    let data = buf[..len].to_vec();
                    let state_inner = Arc::clone(&state_clone);
                    let socket_ref = &socket;
                    // Process inline to avoid borrow issues with socket
                    let response = handle_query(&data, &state_inner);
                    state_inner
                        .query_count
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    match response {
                        Ok(resp_bytes) => {
                            if let Err(e) = socket_ref.send_to(&resp_bytes, src).await {
                                error!("failed to send response to {}: {}", src, e);
                            }
                        }
                        Err(e) => {
                            warn!("failed to handle query from {}: {}", src, e);
                        }
                    }
                }
                Err(e) => {
                    error!("recv_from error: {}", e);
                }
            }
        }
    });

    Ok(state)
}

/// Parse a DNS query and build a response.
fn handle_query(data: &[u8], state: &DnsServerState) -> Result<Vec<u8>> {
    let request = Message::from_vec(data).context("parsing DNS query")?;
    let mut response = Message::new();

    let mut header = Header::response_from_request(request.header());
    header.set_authoritative(true);

    response.set_header(header);

    // Copy the question section
    for query in request.queries() {
        response.add_query(query.clone());
    }

    if request.header().op_code() != OpCode::Query {
        response.set_response_code(ResponseCode::NotImp);
        return Ok(response.to_vec()?);
    }

    for query in request.queries() {
        let name = query.name();
        let qclass_raw: u16 = query.query_class().into();
        let qtype = query.query_type();

        debug!(
            "query: {} class={} type={:?}",
            name, qclass_raw, qtype
        );

        // Only handle HS class (4) or IN class (1) as fallback
        if qclass_raw != DNS_CLASS_HS && qclass_raw != u16::from(DNSClass::IN) {
            continue;
        }

        // Only handle TXT queries
        if qtype != RecordType::TXT {
            continue;
        }

        if let Some(txt_data) = resolve_name(name, &state.zone) {
            let txt_rdata = TXT::new(vec![txt_data.clone()]);
            let mut record = Record::from_rdata(name.clone(), state.zone.ttl, RData::TXT(txt_rdata));
            record.set_dns_class(DNSClass::HS);
            response.add_answer(record);
        } else {
            debug!("no record found for {}", name);
        }
    }

    if response.answers().is_empty() {
        response.set_response_code(ResponseCode::NXDomain);
    }

    Ok(response.to_vec()?)
}

/// Resolve a DNS name against the zone.
/// Expected format: `<key>.<map_type><lhs><rhs>` e.g. `admin.passwd.ns.flatracoon.internal`
fn resolve_name(name: &Name, zone: &HesiodZone) -> Option<String> {
    let name_str = name.to_string();
    // Remove trailing dot if present
    let name_str = name_str.strip_suffix('.').unwrap_or(&name_str);

    // Build the expected suffix: e.g. ".ns.flatracoon.internal"
    let suffix = format!("{}{}", zone.lhs, zone.rhs);

    // Strip the suffix to get "<key>.<map_type>"
    let prefix = name_str.strip_suffix(&suffix)?;

    // Split into key and map_type
    let dot_pos = prefix.rfind('.')?;
    let key = &prefix[..dot_pos];
    let map_label = &prefix[dot_pos + 1..];

    let map_type: MapType = map_label.parse().ok()?;
    let record = zone.lookup(key, map_type)?;

    Some(record.to_txt())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::HesiodConfig;

    fn test_zone() -> HesiodZone {
        let config = HesiodConfig {
            domain: "test.internal".into(),
            lhs: ".ns".into(),
            rhs: ".test.internal".into(),
            ttl: 300,
            dns_port: 53,
            http_port: 8080,
            services: vec![crate::config::ServiceEntry {
                name: "web".into(),
                host: "web.svc".into(),
                port: 443,
                protocol: "tcp".into(),
            }],
            users: vec![],
            groups: vec![],
        };
        HesiodZone::from_config(&config).unwrap()
    }

    #[test]
    fn resolve_service_name() {
        let zone = test_zone();
        let name: Name = "web.service.ns.test.internal".parse().unwrap();
        let result = resolve_name(&name, &zone);
        assert_eq!(result, Some("web.svc:443:tcp".into()));
    }

    #[test]
    fn resolve_missing_name() {
        let zone = test_zone();
        let name: Name = "missing.service.ns.test.internal".parse().unwrap();
        assert!(resolve_name(&name, &zone).is_none());
    }

    #[test]
    fn resolve_wrong_suffix() {
        let zone = test_zone();
        let name: Name = "web.service.ns.other.internal".parse().unwrap();
        assert!(resolve_name(&name, &zone).is_none());
    }
}
