use std::collections::HashMap;
use crate::http1::utils::errors::ParseError;


pub(crate) struct URI {
    host:  String,
    port:  Option<u16>,
    path:  Option<Path>,
    query: Option<Query>,
}
    struct Path(
        // Vec<String>
        String
    );
    struct Query(
        HashMap<String, String>
    );

impl URI {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        if &bytes[0..7] != b"https://" {
            return Err(ParseError::InvalidUri(String::from_utf8_lossy(bytes).into()))
        }

        let eol = bytes.len();
        let mut read_pos = 7;

        let mut uri = Self {
            host:  String::new(),
            port:  None,
            path:  None,
            query: None,
        };
        while read_pos < eol {
            if bytes[read_pos] != b':' && bytes[read_pos] != b'/' {
                uri.host.push(bytes[read_pos] as char);
                read_pos += 1
            }
        }
        if bytes[read_pos] == b':' {
            read_pos += 1;
            let port_num_start = read_pos;
            while bytes[read_pos].is_ascii_digit() {
                read_pos += 1
            }
            if port_num_start == read_pos {
                return Err(ParseError::InvalidUri(format!(
                    "Empty port number"
                )));
            }
            uri.port = Some(
                bytes[port_num_start..read_pos].iter().fold(
                    0u16, |a, b| a * 10 + *b as u16
                )
            )
        }
        if bytes[read_pos] == b'/' {
            let path_start = read_pos;
            while read_pos < eol {
                if bytes[read_pos] != b'?' {
                    read_pos += 1
                }
            }
            uri.path = Some(Path(String::from_utf8_lossy(&bytes[path_start..read_pos]).into()));

            if read_pos < eol - 1 && bytes[read_pos] == b'?' {
                uri.query = Some(
                    Query::from_bytes(&bytes[read_pos+1..eol])?
                )
            }
        }
        Ok(uri)
    }
}
impl Query {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        let eol = bytes.len();
        let mut query = HashMap::new();

        let mut read_pos = 0;
        while read_pos < eol {
            let (key, value) = Self::parse_key_value(bytes, &mut read_pos)?;
            query.insert(key, value);
        }

        Ok(Self(query))
    }
    fn parse_key_value(bytes: &[u8], pos: &mut usize) -> Result<(String, String), ParseError> {
        let eol = bytes.len();
        let (mut key, mut value) = (String::new(), String::new());

        let key_start = *pos;
        while *pos < eol-1 && bytes[*pos] != b'=' {
            *pos += 1
        }
        if *pos == key_start || bytes[*pos] != b'=' {
            return Err(ParseError::InvalidQueryParameter(String::from_utf8_lossy(bytes).into()))
        }
        key = String::from_utf8_lossy(&bytes[0..*pos]).into();

        *pos += 1;

        let value_start = *pos;
        while *pos < eol && bytes[*pos] != b'&' {
            *pos += 1
        }
        if *pos == value_start {
            return Err(ParseError::InvalidQueryParameter(String::from_utf8_lossy(bytes).into()))
        }
        value = String::from_utf8_lossy(&bytes[value_start..*pos]).into();

        if bytes[*pos] == b'&' {*pos += 1}
        Ok((key, value))
    }
}