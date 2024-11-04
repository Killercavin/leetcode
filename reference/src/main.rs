fn referencing_greeting(new_greeting: &String) -> String{
    return new_greeting.to_string();
}

fn main() {
    let greeting = String::from("Hello");

    // calling the reference to the heap
    let referenced_greeting = referencing_greeting(&greeting); // the ampersand is used to reference to a memory location
    println!("{}, from Rust!", referenced_greeting);

    println!("The length of the greeting is {}", greeting.len());
}
