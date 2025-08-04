use std::collections::HashSet;
use crate::error::ContactBookError;
use crate::models::contact::{TagType, Contact};
use crate::utils::is_valid_email;

pub struct ContactBook {
    contacts: Vec<Contact>
}

impl ContactBook {
    pub fn new() -> Self {
        Self { contacts: Vec::new() }
    }

    pub fn find_contact_by_name(&self, name: &str) -> Result<&Contact, ContactBookError> {
        match self.find_contact_index(name) {
            Some(i) => Ok(&self.contacts[i]),
            None => Err(ContactBookError::ContactNotFound)
        }
    }

    pub fn find_contact_by_phone_number(&self, phone_number: &str) -> Result<&Contact, ContactBookError> {
        self.contacts
            .iter()
            .find(|c| c.phone_numbers.iter().any(|n| n == phone_number))
            .ok_or(ContactBookError::ContactNotFound)
    }

    pub fn find_all_contacts_by_email_domain(&self, email_domain: &str) -> Vec<&Contact> {
        let domain = email_domain.to_lowercase();
        self.contacts
            .iter()
            .filter(|c| {
                c.email
                    .rsplit_once("@")
                    .map(|(_, d)| d.eq_ignore_ascii_case(&domain))
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn find_all_by_tags(&self, tag: &TagType) -> Vec<&Contact> {
        self.contacts.iter().filter(|c| c.tags.contains(tag)).collect()
    }

    pub fn add_new_contact(&mut self, name: &str, email: &str, phone_numbers: Option<Vec<&str>>, tags: Option<HashSet<TagType>>) -> Result<&Contact, ContactBookError> {
        if !is_valid_email(email) {
            return Err(ContactBookError::InvalidEmailFormat)
        }
        match self.find_contact_index(name) {
            Some(i) => Ok(&self.contacts[i]),
            None => {
                self.contacts.push(Contact {
                    name: name.to_string(),
                    email: email.to_string(),
                    phone_numbers: match phone_numbers {
                        None => Vec::new(),
                        Some(numbers) => numbers.iter().map(|n| n.to_string()).collect()
                    },
                    tags: tags.unwrap_or_else(|| HashSet::new())
                });
                self.contacts.last().ok_or(ContactBookError::ContactNotFound)
            }
        }
    }

    pub fn remove_contact(&mut self, name: &str) -> Result<Contact, ContactBookError> {
        match self.find_contact_index(name) {
            Some(i) => Ok(self.contacts.remove(i)),
            None => Err(ContactBookError::ContactNotFound),
        }
    }

    pub fn update_contact(&mut self, name: &str, new_email: Option<&str>, new_name: Option<&str>) -> Result<&Contact, ContactBookError> {
        match self.find_contact_mut(name) {
            Some(contact) => {
                if let Some(email) = new_email {
                    contact.email = match is_valid_email(email) {
                        true => email.to_string(),
                        false => Err(ContactBookError::InvalidEmailFormat)?
                    };
                }
                if let Some(new_name) = new_name {
                    contact.name = new_name.to_string();
                }
                Ok(contact)
            },
            None => Err(ContactBookError::ContactNotFound),
        }
    }

    pub fn add_phone_number(&mut self, name: &str, new_number: &str) -> Result<&Contact, ContactBookError> {
        let contact = self
            .find_contact_mut(name)
            .ok_or(ContactBookError::ContactNotFound)?; // '?' either gives me the OK result or immediately returns the Err from this fn
        contact.add_phone_number(new_number)
    }

    pub fn remove_phone_number(&mut self, name: &str, number: &str) -> Result<&Contact, ContactBookError> {
        let contact = self
            .find_contact_mut(name)
            .ok_or(ContactBookError::ContactNotFound)?;
        contact.remove_phone_number(number)
    }

    pub fn add_tag(&mut self, name: &str, tag: TagType) -> Result<&Contact, ContactBookError> {
        let contact = self
            .find_contact_mut(name)
            .ok_or(ContactBookError::ContactNotFound)?;
        contact.add_tag(tag)
    }

    pub fn remove_tag(&mut self, name: &str, tag: &TagType) -> Result<&Contact, ContactBookError> {
        let contact = self
            .find_contact_mut(name)
            .ok_or(ContactBookError::ContactNotFound)?;
        contact.remove_tag(tag)
    }

    fn find_contact_index(&self, name: &str) -> Option<usize> {
        self.contacts.iter().position(|c| c.name == name)
    }

    fn find_contact_mut(&mut self, name: &str) -> Option<&mut Contact> {
        self.contacts.iter_mut().find(|c| c.name == name)
    }
}