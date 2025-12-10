use std::{net::Ipv4Addr, path::PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetTableEntry {
    pub process_id: u32,
    pub process_name: String,
    pub local_port: u16,
    pub local_ip_address: Ipv4Addr,
    pub protocol: ProtocolInfo
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "protocol", rename_all = "lowercase")]
pub enum ProtocolInfo {
    Tcp {
        remote_port: Option<u16>,
        remote_ip_address: Option<Ipv4Addr>,
        state: TcpState,
    },
    Udp,
}

impl From<u32> for TcpState {
    fn from(value: u32) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TcpState {
    Closed = 1,
    Listening = 2,
    SynSent = 3,
    SynReceived = 4,
    Established = 5,
    FinWait1 = 6,
    FinWait2 = 7,
    CloseWait = 8,
    Closing = 9,
    LastAck = 10,
    TimeWait = 11,
    DeleteTcb = 12,
}
