pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.split('@').nth(1).map_or(false, |d| !d.is_empty())
}