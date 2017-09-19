#[derive(Debug)]
pub struct IceCandidate {
    pub foundation: String,
    pub componentId: Option<u32>,
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
    Token,
}
