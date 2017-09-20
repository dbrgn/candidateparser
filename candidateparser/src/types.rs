use std::collections::HashMap;
use std::net::IpAddr;

/// The ICE candidate struct. Contains all data from the SDP.
#[derive(Debug, PartialEq, Eq)]
pub struct IceCandidate {
    pub foundation: String,
    pub component_id: u32,
    pub transport: Transport,
    pub priority: u64,
    pub connection_address: IpAddr,
    pub port: u16,
    pub candidate_type: CandidateType,
    pub rel_addr: Option<IpAddr>,
    pub rel_port: Option<u16>,
    pub extensions: Option<HashMap<Vec<u8>, Vec<u8>>>,
}

/// The transport type. In almost all cases this will be `Transport::Udp`.
#[derive(Debug, PartialEq, Eq)]
pub enum Transport {
    Udp,
    Extension(String)
}

/// All possible candidate types.
#[derive(Debug, PartialEq, Eq)]
pub enum CandidateType {
    Host,
    Srflx,
    Prflx,
    Relay,
    Token(String),
}
