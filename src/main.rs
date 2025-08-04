use std::error::Error;
use std::fmt::Debug;
use crate::book::contact_book::ContactBook;
use crate::generate::generate_contact;
use crate::persistence::file_storage::FileStorage;

mod book;
mod models;
mod persistence;
mod generate;
mod error;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut book = ContactBook::new();
    for _ in 0..5 {
        let contact = generate_contact();
        book.add_contact(contact)?;
    }

    print_contacts(&book);

    let file_storage = FileStorage::new("contacts.json");
    file_storage.save(&book)?;

    let loaded = file_storage.load::<ContactBook>()?;
    print_contacts(&loaded);

    Ok(())
}

fn print_contacts(contact_book: &ContactBook) {
    for contact in &contact_book.contacts {
        println!("{:?}", contact);
    }
}
