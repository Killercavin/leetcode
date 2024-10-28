fn main() {
    println!("Welcome to the world of Rust variables");

    // mutable variables
    let mut x: i32 = 5;
    println!("The value of x is {x}");

    x = 6; // error can't mutate variable x
    println!("The value of x is now {x}");

    // declaring const variables
    const MY_CAR_MODEL: &str = "Volvo";
    println!("My car model is {MY_CAR_MODEL}");
    
    // shadowing a variable
    let number = 2;
    println!("The value of number is {number}");

    // re-declaring the number variable again but now assigning a different value
    let my_string: &str = "My string";
    println!("My string is {my_string}");
    {
        let number = 5 + number;
        println!("The value of number has now changed to {number}");
    }

    println!("The value of number is {number}");

    {
        let spaces = " ";
        let spaces = spaces.len();
        println!("The number of spaces is {spaces}");
    }
    
}
