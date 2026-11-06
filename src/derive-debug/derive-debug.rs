/* All user-defined data types/structures doesn't have fmt::Display,
 * you need to create it manually.
 *
 * For debugging purposes you can derive #[derive(Debug)]
 * then use {:?} for formatted printing debug info.
*/

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Marcos";
    let age: u8 = 22;
    let marcos = Person {name, age};

    // This doesn't work, because struct Person doesn't have fmt::Display implemented.
    // println!("{}", marcos);

    // This works, because the struct derives Debug. we use {:#?} for pretty-printing info.
    println!("{:#?}", marcos);

    // This works, because the fields of the struct are standard rust data types.
    println!("My name is {name} and I'm {age} years old!", name = marcos.name, age = marcos.age);
}
