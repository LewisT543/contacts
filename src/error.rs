#[derive(Debug)]
pub enum ContactBookError {
    ContactNotFound,
    DuplicatePhoneNumber,
    PhoneNumberNotFound,
    DuplicateTag,
    TagNotFound,
    InvalidEmailFormat
}