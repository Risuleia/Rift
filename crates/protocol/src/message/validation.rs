use crate::MessageError;

pub(super) fn validate_text_field<F>(
    value: &str,
    maximum: usize,
    empty_error: MessageError,
    too_long_error: F,
    invalid_error: MessageError,
) -> Result<(), MessageError>
where
    F: FnOnce(usize, usize) -> MessageError,
{
    if value.is_empty() {
        return Err(empty_error);
    }

    if value.len() > maximum {
        return Err(too_long_error(value.len(), maximum));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_error);
    }

    Ok(())
}
