use hyper::header::HeaderValue;
use std::time::Duration;

const ZERO: u8 = 48; // '0'.to_digit(10).unwrap() as u8;
const NINE: u8 = 57; // '9'.to_digit(10).unwrap() as u8;
const COMMA: u8 = 44; // ','.to_digit(10).unwrap() as u8;

#[derive(Debug)]
pub struct Payload {
    pub response_size: u32,
    pub sleep_time: Duration,
}

impl Payload {
    pub fn from_header(val: Option<&HeaderValue>) -> Result<Payload, String> {
        if val.is_none() {
            return Err(String::from("Custom Attributes header not present"));
        }
        let bytes = val.unwrap().as_bytes();
        let mut comma_found = false;
        let mut response_size: i32 = -1;
        let mut sleep_time_millis: i32 = -1;
        for byte in bytes {
            let byte = *byte;
            if byte == COMMA {
                if comma_found {
                    return Err(String::from("Comma already found"));
                }
                comma_found = true;
                continue;
            }
            if byte < ZERO || byte > NINE {
                return Err(format!("Invalid char found: '{}'", byte as char));
            }
            let num = (byte - ZERO) as i32;
            if comma_found {
                response_size = (response_size >= 0) as i32 * response_size * 10 + num;
            } else {
                sleep_time_millis = (sleep_time_millis >= 0) as i32 * sleep_time_millis * 10 + num;
            }
        }
        if sleep_time_millis < 0 || response_size < 0 {
            return Err(String::from(
                "Must specify both response size and sleep time separated by a comma",
            ));
        }
        Ok(Payload {
            response_size: response_size as u32,
            sleep_time: Duration::from_millis(sleep_time_millis as u64),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Payload;
    use hyper::http::HeaderValue;
    use std::time::Duration;

    #[test]
    fn test_payload_from_header() {
        let val = HeaderValue::from_static("9,13");
        let payload = Payload::from_header(Some(&val));
        let payload = payload.unwrap();
        assert_eq!(payload.sleep_time, Duration::from_millis(9));
        assert_eq!(payload.response_size, 13 as u32);
    }

    #[test]
    fn test_payload_from_header_large_numbers() {
        let val = HeaderValue::from_static("982411011,1300001");
        let payload = Payload::from_header(Some(&val));
        let payload = payload.unwrap();
        assert_eq!(payload.sleep_time, Duration::from_millis(982411011));
        assert_eq!(payload.response_size, 1300001 as u32);
    }

    #[test]
    fn test_payload_from_header_zeros() {
        let val = HeaderValue::from_static("0,0");
        let payload = Payload::from_header(Some(&val));
        let payload = payload.unwrap();
        assert_eq!(payload.sleep_time, Duration::from_millis(0));
        assert_eq!(payload.response_size, 0 as u32);
    }

    #[test]
    fn test_payload_from_header_empty() {
        let payload = Payload::from_header(None);
        assert!(payload.is_err())
    }

    #[test]
    fn test_payload_from_sleep_time_empty() {
        let val = HeaderValue::from_static(",877");
        let payload = Payload::from_header(Some(&val));
        assert!(payload.is_err())
    }

    #[test]
    fn test_payload_from_response_size_empty() {
        let val = HeaderValue::from_static("934,");
        let payload = Payload::from_header(Some(&val));
        assert!(payload.is_err())
    }

    #[test]
    fn test_payload_from_only_comma() {
        let val = HeaderValue::from_static(",");
        let payload = Payload::from_header(Some(&val));
        assert!(payload.is_err())
    }

    #[test]
    fn test_payload_from_bad_char() {
        let val = HeaderValue::from_static("a,99");
        let payload = Payload::from_header(Some(&val));
        assert!(payload.is_err())
    }

    #[test]
    fn test_payload_from_two_commas() {
        let val = HeaderValue::from_static("22,99,333");
        let payload = Payload::from_header(Some(&val));
        assert!(payload.is_err())
    }
}
