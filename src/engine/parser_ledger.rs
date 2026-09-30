use std::num::NonZeroU32;
use super::error_types::ParseLedgerError;

pub fn parse_ledger(ledger: &str) -> Result<NonZeroU32, ParseLedgerError> {
    let ledger_u32: u32 = ledger.parse::<u32>().map_err(|_| ParseLedgerError::InvalidValue{ value: ledger.to_string() })?;
    NonZeroU32::new(ledger_u32).ok_or(ParseLedgerError::ZeroValue)
}
