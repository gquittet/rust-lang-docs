#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Classic print
    println!("Classic print {:?}", peter);
    // Pretty print
    println!("Pretty print {:#?}", peter);
}

