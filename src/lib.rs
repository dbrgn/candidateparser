//! ICE Candidate parser following RFC5245.

#[macro_use]
extern crate nom;

pub mod parsers;
pub mod types;

/*pub fn parse(sdp: &str) -> Result<IceCandidate, String> {
    Err("oh".to_string())
}*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
