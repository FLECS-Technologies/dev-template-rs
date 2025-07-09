use std::num::ParseIntError;
use tracing::{debug, info_span, trace};

#[derive(thiserror::Error, Debug)]
pub enum AddError {
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error("Adding {left} and {right} would result in an overflow")]
    Overflow { left: u64, right: u64 },
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Could not parse {s}: {e}")]
    ParseError { s: String, e: ParseIntError },
}

fn parse_str(s: &str, radix: u32) -> Result<u64, ParseError> {
    u64::from_str_radix(s, radix).map_err(|e| ParseError::ParseError {
        s: s.to_string(),
        e,
    })
}

pub fn try_add_strs(left: &str, right: &str, radix: u32) -> Result<u64, AddError> {
    let _span = info_span!("add", left, right).entered();
    let left = parse_str(left, radix)?;
    trace!("parsed {left}");
    let right = parse_str(right, radix)?;
    trace!("parsed {right}");
    debug!("Adding {left} and {right}");
    left.checked_add(right)
        .ok_or(AddError::Overflow { left, right })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn parse_hex_str_ok() {
        assert_eq!(parse_str("73bf", 16).unwrap(), 0x73bf);
    }

    #[test]
    fn parse_hex_str_err() {
        let input = "7$3bf";
        let ParseError::ParseError { s, e } = parse_str(input, 16).err().unwrap();
        assert_eq!(s, input);
        assert_eq!(*e.kind(), IntErrorKind::InvalidDigit);
    }

    #[test]
    fn try_add_hex_strs_ok() {
        assert_eq!(try_add_strs("25f0", "150f", 16).unwrap(), 0x3aff);
    }

    #[test]
    fn try_add_hex_strs_err_left() {
        assert!(matches!(
            try_add_strs("25&f0", "150f", 16),
            Err(AddError::ParseError(_))
        ));
    }

    #[test]
    fn try_add_hex_strs_err_right() {
        assert!(matches!(
            try_add_strs("25f0", "15/0f", 16),
            Err(AddError::ParseError(_))
        ));
    }

    #[test]
    fn try_add_hex_strs_err_overflow() {
        assert!(matches!(
            try_add_strs("ffffffffffffffff", "150f", 16),
            Err(AddError::Overflow {
                left: 0xffffffffffffffff,
                right: 0x150f
            })
        ));
    }
}
