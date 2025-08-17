 use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person1 = Person {
        name: String::from("Mayank"),
        age: 22,
    };

    let person2 = person1.clone(); // Thanks to Clone trait

    println!("{:?}", person2);     // Thanks to Debug trait
}
