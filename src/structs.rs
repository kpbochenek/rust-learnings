use std::fmt::Display;

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    male: bool,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("User({}|{}|{})", self.name, self.age, self.male))
    }
}

pub fn structs() {
    println!("Structs examples  ");
    println!("----------------");

    let tom = User {
        name: String::from("Tom"),
        age: 15,
        male: true,
    };
    println!("Tom {}", tom);
    println!("Tom {:?}", tom);
    println!("Tom pretty {:#?}", tom);
}
