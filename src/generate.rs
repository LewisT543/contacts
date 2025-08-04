use std::collections::HashSet;
use crate::models::contact::{Contact, TagType};
use rand::Rng;
use maplit::hashset;

pub fn generate_x_contacts(num_contacts: i32) -> Vec<Contact> {
    (0..num_contacts).map(|_| generate_contact()).collect()
}

pub fn generate_contact() -> Contact {
    let f_name = get_random_item(&FIRST_NAMES).to_string();
    let l_name = get_random_item(&LAST_NAMES).to_string();
    Contact {
        name: format!("{} {}", f_name, l_name),
        email: format!("{}{}{}", f_name, l_name, get_random_item(&DOMAINS).to_string()),
        tags: get_random_tags(),
        phone_numbers: generate_phone_numbers()
    }
}

fn get_random_item<T>(vec: &[T]) -> &T {
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..vec.len());
    &vec[random_index]
}

fn get_random_tags() -> HashSet<TagType> {
    let mut rng = rand::rng();
    let random_num = rng.random_range(0..=10);
    match random_num {
        n if n <= 5 => HashSet::new(),
        n if n == 6 => hashset!{TagType::Hobbies},
        n if n == 7 => hashset!{TagType::Work},
        n if n == 8 => hashset!{TagType::Home},
        n if n == 9 => hashset!{TagType::Work, TagType::Hobbies},
        _ => hashset!{TagType::Work, TagType::Hobbies, TagType::Home},
    }
}

fn generate_phone_numbers() -> Vec<String> {
    let mut rng = rand::rng();
    (0..rng.random_range(0..=3))
        .map(|_| generate_single_phone_number())
        .collect()
}

fn generate_single_phone_number() -> String {
    let mut rng = rand::rng();
    let area_code = rng.random_range(100..1000);
    let prefix = rng.random_range(100..1000);
    let line_number = rng.random_range(1000..10000);
    format!("({}) {}-{}", area_code, prefix, line_number)
}

const FIRST_NAMES: [&str; 10] = ["Alice", "Bob", "Charlie", "Diana", "Ethan", "Fiona", "George", "Hannah", "Ian", "Julia"];
const LAST_NAMES: [&str; 10] = ["Smith", "Johnson", "Williams", "Brown", "Jones", "Miller", "Davis", "Garcia", "Rodriguez", "Wilson"];
const DOMAINS: [&str; 3] = ["@busycompany.com", "@gmail.com", "@yahoo.co.uk"];
