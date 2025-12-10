use tauri::{command, State};
use log::*;
use crate::{models::{GetNetTableArgs, NetTableEntry, PageArgs, Paged, ProtocolInfo, TransportProtocol}, services::*};

use super::error::*;

#[command]
pub fn get_net_table(net_table_service: State<NetTableService>, args: GetNetTableArgs) -> Result<Paged<NetTableEntry>> {

    let GetNetTableArgs {
        page: PageArgs { page, page_size },
        protocols,
        local_ip_addr,
        local_port,
        process_name,
        remote_ip_addr,
        remote_port
    } = args;

    let mut entries = Vec::new();

    if protocols.contains(&TransportProtocol::Tcp) {
        entries.extend(net_table_service.get_tcp_table()?);
    }

    if protocols.contains(&TransportProtocol::Udp) {
        entries.extend(net_table_service.get_udp_table()?);
    }

    let entries: Vec<NetTableEntry> = entries.into_iter()
        .filter(|e| protocols.contains(match &e.protocol {
            ProtocolInfo::Tcp { .. } => &TransportProtocol::Tcp,
            ProtocolInfo::Udp => &TransportProtocol::Udp,
        }))
        .filter(|e| local_ip_addr.map_or(true, |ip| e.local_ip_address == ip))
        .filter(|e| local_port.map_or(true, |p| e.local_port == p))
        .filter(|e| process_name.as_ref().map_or(true, |name| e.process_name.contains(name)))
        .filter(|e| match &e.protocol {
            ProtocolInfo::Tcp { remote_ip_address, remote_port, .. } => {
                remote_ip_addr.map_or(true, |ip| remote_ip_address.map_or(false, |r| r == ip)) &&
                remote_port.map_or(true, |p| remote_port.map_or(false, |r| r == p))
            },
            ProtocolInfo::Udp => remote_ip_addr.is_none() && remote_port.is_none(),
        })
        .collect();

    let total = entries.len() as u32;
    let start = (page * page_size) as usize;
    let end = (start + page_size as usize).min(entries.len());
    let items = if start < entries.len() { entries[start..end].to_vec() } else { Vec::new() };

    Ok(Paged {
        items,
        page,
        page_size,
        total
    })
}