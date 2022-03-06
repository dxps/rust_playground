use std::rc::Rc;

#[derive(Debug)]
struct Pet {
    name: String,
}

impl Pet {
    fn new(name: String) -> Self {
        Self { name }
    }
}

struct Person {
    pets: Vec<Rc<Pet>>,
}

fn main() {
    // Create two pets with shared ownership
    let cat = Rc::new(Pet::new("Tigger".into()));
    let dog = Rc::new(Pet::new("Chase".into()));

    // Create one person who owns both pets
    let brother = Person {
        pets: vec![cat.clone(), dog.clone()],
    };

    // Create another person who _also_ owns both pets
    let sister = Person {
        pets: vec![cat, dog],
    };

    // Even if one person gives up ownership, the other person
    // still has shared ownership, so the pets are kept around (yay!)
    drop(sister);
    println!("Pets: {:?}", brother.pets);

    // ___________________________________________________
    // Just to get rid of the field name not used warning.
    let _ = brother
        .pets
        .first()
        .expect("brother has pets")
        .name
        .as_str();
}
