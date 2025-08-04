use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ContactBookError {
    ContactAlreadyExists,
    ContactNotFound,
    DuplicatePhoneNumber,
    PhoneNumberNotFound,
    DuplicateTag,
    TagNotFound,
    InvalidEmailFormat
}

impl fmt::Display for ContactBookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ContactBookError::*;
        let msg = match self {
            ContactNotFound => "Contact not found",
            DuplicatePhoneNumber => "Duplicate phone number",
            PhoneNumberNotFound => "Phone number not found",
            DuplicateTag => "Duplicate tag",
            TagNotFound => "Tag not found",
            InvalidEmailFormat => "Invalid email format",
            ContactAlreadyExists => "Contact already exists",
        };
        write!(f, "{msg}")
    }
}

impl Error for ContactBookError {}