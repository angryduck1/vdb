use super::error_types::ValuesError;

pub fn is_valid_name(i: usize, name: &str) -> Result<(), ValuesError> {
    if name.len() == 0 {
        return Err(ValuesError::EmptyName{index: i});
    }

    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        _ => return Err(ValuesError::InvalidName{name: name.to_string()}),
    }

    if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(ValuesError::InvalidName{name: name.to_string()})
    }

    Ok(())
}
