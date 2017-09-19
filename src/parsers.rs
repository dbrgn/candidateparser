//! Parsers for parsing the ICE candidates.
//! 
//! **Note:** These parsers are not stable and may be changed at any time.
use std::str;

use nom::{is_alphanumeric, digit, alpha};

use ::types::Transport;


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

#[cfg(test)]
mod tests {
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
}
