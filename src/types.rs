use std::net::IpAddr;

#[derive(Debug)]
pub struct IceCandidate {
    pub foundation: String,
    pub component_id: u32,
    pub transport: Transport,
    pub priority: u64,
    pub connection_address: IpAddr,
    pub port: u16,
    pub candidate_type: CandidateType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Transport {
    Udp,
    Extension(String)
}

#[derive(Debug, PartialEq, Eq)]
pub enum CandidateType {
    Host,
    Srflx,
    Prflx,
    Relay,
    Token(String),
}
