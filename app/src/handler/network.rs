use tauri::{command, State};
use log::*;
use crate::{models::{GetNetTableArgs, NetTableEntry, PageArgs, Paged}, services::*};

use super::error::*;

#[command]
pub fn get_tcp_table(net_table_service: State<NetTableService>, args: GetNetTableArgs) -> Result<Paged<NetTableEntry>> {

    let GetNetTableArgs { 
        page: PageArgs { page, page_size },
        local_ip_addr,
        local_port,
        process_name,
        remote_ip_addr,
        remote_port
    } = args;

    let entries = net_table_service.get_tcp_table()?;
    let entries = net_table_service.get_udp_table()?;

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