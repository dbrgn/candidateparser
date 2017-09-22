use std::convert::Into;
use std::collections::HashMap;
use std::ffi::CString;
use std::net::IpAddr;

/// The ICE candidate struct. Contains all data from the SDP.
#[derive(Debug, PartialEq, Eq, Clone)]
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
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Transport {
    Udp,
    Extension(String)
}

impl Into<CString> for Transport {
    fn into(self) -> CString {
        match self {
            Transport::Udp => CString::new("udp").unwrap(),
            Transport::Extension(e) => CString::new(e).unwrap(),
        }
    }
}

/// All possible candidate types.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum CandidateType {
    Host,
    Srflx,
    Prflx,
    Relay,
    Token(String),
}

impl Into<CString> for CandidateType {
    fn into(self) -> CString {
        match self {
            CandidateType::Host => CString::new("host").unwrap(),
            CandidateType::Srflx => CString::new("srflx").unwrap(),
            CandidateType::Prflx => CString::new("prflx").unwrap(),
            CandidateType::Relay => CString::new("relay").unwrap(),
            CandidateType::Token(e) => CString::new(e).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_into_cstring() {
        let converted1: CString = Transport::Udp.into();
        assert_eq!(converted1, CString::new("udp").unwrap());

        let converted2: CString = Transport::Extension("yolo".into()).into();
        assert_eq!(converted2, CString::new("yolo").unwrap());
    }

    #[test]
    fn test_candidate_type_into_cstring() {
        let host: CString = CandidateType::Host.into();
        let srflx: CString = CandidateType::Srflx.into();
        let prflx: CString = CandidateType::Prflx.into();
        let relay: CString = CandidateType::Relay.into();
        let token: CString = CandidateType::Token("Yolo".into()).into();

        assert_eq!(host, CString::new("host").unwrap());
        assert_eq!(srflx, CString::new("srflx").unwrap());
        assert_eq!(prflx, CString::new("prflx").unwrap());
        assert_eq!(relay, CString::new("relay").unwrap());
        assert_eq!(token, CString::new("Yolo").unwrap());
    }
}
