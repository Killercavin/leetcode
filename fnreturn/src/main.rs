fn my_name(name: &str) -> &str{
    name
}

fn main() {
    let name = my_name("Rust");
    println!("My name is {name}");
}


// this program shows how to return values from a function with example being how to explicitly return a string