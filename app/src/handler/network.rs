use tauri::{command, State};
use log::*;
use crate::{models::{PageArgs, Paged, TcpTableEntry}, services::*};

use super::error::*;

#[command]
pub fn get_tcp_table(tcp_table_service: State<TcpTableService>, args: PageArgs) -> Result<Paged<TcpTableEntry>> {

    let PageArgs { page, page_size } = args;

    let entries = tcp_table_service.get_tcp_table()?;

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