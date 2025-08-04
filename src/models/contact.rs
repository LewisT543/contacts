use std::collections::HashSet;

use crate::error::ContactBookError;


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TagType {
    Work,
    Home,
    Hobbies
}

impl std::fmt::Display for TagType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            TagType::Work => "Work",
            TagType::Home => "Home",
            TagType::Hobbies => "Hobbies",
        };
        write!(f, "{}", s)
    }
}

pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone_numbers: Vec<String>,
    pub tags: HashSet<TagType>
}

impl Contact {
    pub fn add_phone_number(&mut self, new_number: &str) -> Result<&Contact, ContactBookError> {
        match self.find_number_index(new_number) {
            Some(_) => Err(ContactBookError::DuplicatePhoneNumber),
            None => {
                self.phone_numbers.push(new_number.to_string());
                Ok(self)
            }
        }
    }

    pub fn remove_phone_number(&mut self, number: &str) -> Result<&Contact, ContactBookError> {
        match self.find_number_index(number) {
            Some(i) => {
                self.phone_numbers.remove(i);
                Ok(self)
            }
            None => Err(ContactBookError::PhoneNumberNotFound)
        }
    }

    pub fn add_tag(&mut self, tag: TagType) -> Result<&Contact, ContactBookError> {
        if self.tags.insert(tag) {
            Ok(self)
        } else {
            Err(ContactBookError::DuplicateTag)
        }
    }

    pub fn remove_tag(&mut self, tag: &TagType) -> Result<&Contact, ContactBookError> {
        if self.tags.remove(tag) {
            Ok(self)
        } else {
            Err(ContactBookError::TagNotFound)
        }
    }

    pub fn find_number_index(&mut self, number: &str) -> Option<usize> {
        self.phone_numbers.iter().position(|n| n.eq(number))
    }
}