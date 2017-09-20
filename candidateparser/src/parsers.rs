//! Nom parsers for parsing the ICE candidates.
//! 
//! **Note:** These parsers are not stable and may be changed at any time.
use std::collections::HashMap;
use std::str;
use std::net;

use nom::{is_alphanumeric, is_hex_digit, digit, alpha, space};

use ::types::{Transport, CandidateType, IceCandidate};


/// Return whether the specified byte is a valid ice-char.
/// 
/// RFC5245 grammar:
/// 
/// ```ebnf,ignore
/// ice-char = ALPHA / DIGIT / "+" / "/"
/// ```
fn is_ice_char(c: u8) -> bool {
    is_alphanumeric(c) || c == b'+' || c == b'/'
}

/// Return whether the specified byte is a valid IP char.
/// ```
fn is_ip_char(c: u8) -> bool {
    is_hex_digit(c) || c == b'.' || c == b':'
}

named_attr!(#[doc = "
Parse one or more ice-chars.
"],
    pub ice_chars,
    take_while1!(is_ice_char)
);

named_attr!(#[doc = "
Parse the foundation.

RFC5245 grammar:

```ebnf,ignore
foundation = 1*32ice-char
ice-char   = ALPHA / DIGIT / \"+\" / \"/\"
```"],
    pub foundation<&str>,
    map_res!(
        verify!(
            ice_chars,
            |val: &[u8]| val.len() >= 1 && val.len() <= 32
        ),
        str::from_utf8
    )
);


named_attr!(#[doc = "
Parse the component-id.

RFC5245 grammar:

```ebnf,ignore
component-id = 1*5DIGIT
```
"],
    pub component_id<u32>,
    map_res!(
        map_res!(
            verify!(
                digit,
                |val: &[u8]| val.len() >= 1 && val.len() <= 5
            ),
            str::from_utf8
        ),
        str::FromStr::from_str
    )
);

named_attr!(#[doc = "
Parse the transport.

RFC5245 grammar:

```ebnf,ignore
transport           = \"UDP\" / transport-extension
transport-extension = token ; from RFC 3261
```
"],
    pub transport<Transport>,
    map!(
        map_res!(
            alpha,
            str::from_utf8
        ),
        |val: &str| {
            match val {
                "udp" | "Udp" | "uDp" | "udP" | "uDP" | "UdP" | "UDp" | "UDP" => Transport::Udp,
                _ => Transport::Extension(val.to_string())
            }
        }
    )
);

named_attr!(#[doc = "
Parse the priority.

RFC5245 grammar:

```ebnf,ignore
priority = 1*10DIGIT
```
"],
    pub priority<u64>,
    map_res!(
        map_res!(
            verify!(
                digit,
                |val: &[u8]| val.len() >= 1 && val.len() <= 10
            ),
            str::from_utf8
        ),
        str::FromStr::from_str
    )
);

named_attr!(#[doc = "
Parse the cand-type.

RFC5245 grammar:

```ebnf,ignore
cand-type       = \"typ\" SP candidate-types
candidate-types = \"host\" / \"srflx\" / \"prflx\" / \"relay\" / token
```
"],
    pub cand_type<CandidateType>,
    do_parse!(
        tag!("typ") >>
        space >>
        cand_type: map!(
            map_res!(
                alpha,
                str::from_utf8
            ),
            |val: &str| match val {
                "host" => CandidateType::Host,
                "srflx" => CandidateType::Srflx,
                "prflx" => CandidateType::Prflx,
                "relay" => CandidateType::Relay,
                _ => CandidateType::Token(val.into()),
            }
        ) >>
        (cand_type)
    )
);


named_attr!(#[doc = "
Parse an IP address.

The parsing is done using `std::net::IpAddr`.
"],
    pub ip_addr<net::IpAddr>,
    map_res!(
        map_res!(
            take_while1!(is_ip_char),
            str::from_utf8
        ),
        str::FromStr::from_str
    )
);

named_attr!(#[doc = "
Parse a port number.
"],
    pub port<u16>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        str::FromStr::from_str
    )
);

named_attr!(#[doc = "
Parse the rel-addr.

RFC5245 grammar:

```ebnf,ignore
rel-addr = \"raddr\" SP connection-address
```
"],
    pub rel_addr<net::IpAddr>,
    do_parse!(
        tag!("raddr") >>
        space >>
        ip: ip_addr >>
        (ip)
    )
);

named_attr!(#[doc = "
Parse the rel-port.

RFC5245 grammar:

```ebnf,ignore
rel-port = \"rport\" SP port
```
"],
    pub rel_port<u16>,
    do_parse!(
        tag!("rport") >>
        space >>
        p: port >>
        (p)
    )
);

named_attr!(#[doc = "
Parse an extension pair into a tuple.

RFC5245 grammar:

```ebnf,ignore
extension           = SP extension-att-name SP extension-att-value
extension-att-name  = byte-string  ;from RFC 4566
extension-att-value = byte-string
```

Note that this grammar is ambiguous, since the values are delimited by space
but byte-string includes space. This was already reported here
https://www.ietf.org/mail-archive/web/mmusic/current/msg06923.html but with no
reaction. For this parsing step, I'll simply assume that space is not a valid
byte-string character.
"],
    pub extension<(&[u8], &[u8])>,
    do_parse!(
        space >>
        key: take_till1!(|v: u8| v == 0x00 || v == 0x0a || v == 0x0d || v == 0x20) >>
        space >>
        val: take_till1!(|v: u8| v == 0x00 || v == 0x0a || v == 0x0d || v == 0x20) >>
        (key, val)
    )
);

named_attr!(#[doc = "
Parse the entire ICE candidate.

RFC5245 grammar:

```ebnf,ignore
candidate-attribute = \"candidate\" \":\" foundation SP component-id SP
                      transport SP
                      priority SP
                      connection-address SP  ;from RFC 4566
                      port                   ;port from RFC 4566
                      SP cand-type
                      [SP rel-addr]
                      [SP rel-port]
                      *(SP extension-att-name SP
                           extension-att-value)
```
"],
    pub ice_candidate<IceCandidate>,
    do_parse!(
        tag!("candidate:") >>
        foundation: foundation >> space >>
        component_id: component_id >> space >>
        transport: transport >> space >>
        priority: priority >> space >>
        connection_address: ip_addr >> space >>
        port: port >> space >>
        cand_type: cand_type >>
        rel_addr: opt!(
            complete!( // https://github.com/Geal/nom/issues/406
                do_parse!(
                    space >>
                    addr: rel_addr >>
                    (addr)
                )
            )
        ) >>
        rel_port: opt!(
            complete!(
                do_parse!(
                    space >>
                    port: rel_port >>
                    (port)
                )
            )
        ) >>
        extensions: map!(
            many0!(extension),
            |extensions: Vec<(&[u8], &[u8])>| {
                if extensions.len() == 0 {
                    return None;
                }
                let mut extension_map = HashMap::new();
                for (k, v) in extensions {
                    extension_map.insert(k.to_vec(), v.to_vec());
                };
                Some(extension_map)
            }
        ) >>
        (
            IceCandidate {
                foundation: foundation.to_string(),
                component_id: component_id,
                transport: transport,
                priority: priority,
                connection_address: connection_address,
                port: port,
                candidate_type: cand_type,
                rel_addr: rel_addr,
                rel_port: rel_port,
                extensions: extensions,
            }
        )
    )
);


#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use nom::{IResult, ErrorKind};

    use super::*;

    #[test]
    fn test_ice_chars() {
        let empty = &b""[..];

        assert_eq!(ice_chars(&b"a"[..]), IResult::Done(empty, &b"a"[..]));
        assert_eq!(ice_chars(&b"Z"[..]), IResult::Done(empty, &b"Z"[..]));
        assert_eq!(ice_chars(&b"abc+123/XYZ"[..]), IResult::Done(empty, &b"abc+123/XYZ"[..]));
        assert_eq!(ice_chars(&b"ab cd"[..]), IResult::Done(&b" cd"[..], &b"ab"[..]));

        assert_eq!(ice_chars(&b"-"[..]), IResult::Error(ErrorKind::TakeWhile1));
    }

    #[test]
    fn test_foundation() {
        let empty = &b""[..];

        assert_eq!(foundation(&b"abc+/"[..]), IResult::Done(empty, "abc+/"));
        assert_eq!(foundation(&b"abc+/ hello"[..]), IResult::Done(&b" hello"[..], "abc+/"));
        assert_eq!(foundation(&b"01234567890123456789012345678901"[..]), IResult::Done(empty, "01234567890123456789012345678901"));

        assert_eq!(foundation(&b"-abc+/"[..]), IResult::Error(ErrorKind::TakeWhile1));
        assert_eq!(foundation(&b"012345678901234567890123456789012"[..]), IResult::Error(ErrorKind::Verify));
    }

    #[test]
    fn test_component_id() {
        let empty = &b""[..];

        assert_eq!(component_id(&b"0"[..]), IResult::Done(empty, 0));
        assert_eq!(component_id(&b"1"[..]), IResult::Done(empty, 1));
        assert_eq!(component_id(&b"12345"[..]), IResult::Done(empty, 12345));
        assert_eq!(component_id(&b"1234a"[..]), IResult::Done(&b"a"[..], 1234));

        assert_eq!(component_id(&b"123456"[..]), IResult::Error(ErrorKind::Verify));
    }

    #[test]
    fn test_transport() {
        let empty = &b""[..];

        assert_eq!(transport(&b"udp"[..]), IResult::Done(empty, Transport::Udp));
        assert_eq!(transport(&b"UDP"[..]), IResult::Done(empty, Transport::Udp));
        assert_eq!(transport(&b"Udp"[..]), IResult::Done(empty, Transport::Udp));

        assert_eq!(transport(&b"foo bar"[..]), IResult::Done(&b" bar"[..], Transport::Extension("foo".to_string())));
    }

    #[test]
    fn test_priority() {
        let empty = &b""[..];

        assert_eq!(priority(&b"0"[..]), IResult::Done(empty, 0));
        assert_eq!(priority(&b"1"[..]), IResult::Done(empty, 1));
        assert_eq!(priority(&b"1234567890"[..]), IResult::Done(empty, 1234567890));

        assert_eq!(priority(&b"12345678901"[..]), IResult::Error(ErrorKind::Verify));
    }

    #[test]
    fn test_cand_type() {
        let empty = &b""[..];

        assert_eq!(cand_type(&b"typ host"[..]), IResult::Done(empty, CandidateType::Host));
        assert_eq!(cand_type(&b"typ      srflx"[..]), IResult::Done(empty, CandidateType::Srflx));
        assert_eq!(cand_type(&b"typ prflx"[..]), IResult::Done(empty, CandidateType::Prflx));
        assert_eq!(cand_type(&b"typ relay"[..]), IResult::Done(empty, CandidateType::Relay));
        assert_eq!(cand_type(&b"typ footok"[..]), IResult::Done(empty, CandidateType::Token("footok".into())));

        assert_eq!(cand_type(&b"typhost"[..]), IResult::Error(ErrorKind::Space));
    }

    #[test]
    fn test_ip_addr() {
        let empty = &b""[..];

        assert_eq!(ip_addr(&b"127.0.0.1"[..]), IResult::Done(empty, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
        assert_eq!(ip_addr(&b"10.20.30.40"[..]), IResult::Done(empty, IpAddr::V4(Ipv4Addr::new(10, 20, 30, 40))));
        assert_eq!(ip_addr(&b"::1"[..]), IResult::Done(empty, IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))));

        assert_eq!(ip_addr(&b"127001"[..]), IResult::Error(ErrorKind::MapRes));
        assert_eq!(ip_addr(&b"127.0.01"[..]), IResult::Error(ErrorKind::MapRes));
        assert_eq!(ip_addr(&b"127.0.0.0.1"[..]), IResult::Error(ErrorKind::MapRes));
    }

    #[test]
    fn test_rel_addr() {
        let empty = &b""[..];
        assert_eq!(rel_addr(&b"raddr 1.2.3.4"[..]), IResult::Done(empty, IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))));
        assert_eq!(rel_addr(&b"raddr ::1"[..]), IResult::Done(empty, IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))));
    }

    #[test]
    fn test_port() {
        let empty = &b""[..];

        assert_eq!(port(&b"1"[..]), IResult::Done(empty, 1u16));
        assert_eq!(port(&b"32000"[..]), IResult::Done(empty, 32000u16));
        assert_eq!(port(&b"65535"[..]), IResult::Done(empty, 65535u16));

        assert_eq!(port(&b"65536"[..]), IResult::Error(ErrorKind::MapRes));
        assert_eq!(port(&b"655361"[..]), IResult::Error(ErrorKind::MapRes));
    }

    #[test]
    fn test_rel_port() {
        let empty = &b""[..];
        assert_eq!(rel_port(&b"rport 32000"[..]), IResult::Done(empty, 32000u16));
        assert_eq!(rel_port(&b"rport 32000 foo"[..]), IResult::Done(&b" foo"[..], 32000u16));
    }

    #[test]
    fn test_extension() {
        let empty = &b""[..];
        assert_eq!(extension(&b" foo bar"[..]), IResult::Done(empty, (&b"foo"[..], &b"bar"[..])));
    }

    #[test]
    fn test_parse_minimal() {
        let candidate = b"candidate:373990095 1 udp 41885439 5.148.189.205 63293 typ relay";
        let parsed = ice_candidate(&candidate[..]).to_result().unwrap();
        assert_eq!(parsed.foundation, "373990095".to_string());
        assert_eq!(parsed.component_id, 1);
        assert_eq!(parsed.transport, Transport::Udp);
        assert_eq!(parsed.priority, 41885439);
        assert_eq!(parsed.connection_address, IpAddr::V4(Ipv4Addr::new(5, 148, 189, 205)));
        assert_eq!(parsed.port, 63293);
        assert_eq!(parsed.candidate_type, CandidateType::Relay);
        assert_eq!(parsed.rel_addr, None);
        assert_eq!(parsed.rel_port, None);
        assert!(parsed.extensions.is_none());
    }

    #[test]
    fn test_parse_with_rel() {
        let candidate1 = b"candidate:373990095 1 udp 41885439 5.148.189.205 63293 typ relay raddr 1.2.3.4 rport 5432";
        let parsed1 = ice_candidate(&candidate1[..]).to_result().unwrap();
        assert_eq!(parsed1.rel_addr, Some(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))));
        assert_eq!(parsed1.rel_port, Some(5432));

        let candidate2 = b"candidate:373990095 1 udp 41885439 5.148.189.205 63293 typ relay raddr 1.2.3.4";
        let parsed2 = ice_candidate(&candidate2[..]).to_result().unwrap();
        assert_eq!(parsed2.rel_addr, Some(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))));
        assert_eq!(parsed2.rel_port, None);

        let candidate3 = b"candidate:373990095 1 udp 41885439 5.148.189.205 63293 typ relay rport 1337";
        let parsed3 = ice_candidate(&candidate3[..]).to_result().unwrap();
        assert_eq!(parsed3.rel_addr, None);
        assert_eq!(parsed3.rel_port, Some(1337));
    }
}
