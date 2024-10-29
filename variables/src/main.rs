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

    let decimal_num = 3.2;
    let alt_decimal_num: f32 = 3.2;
    let sum = decimal_num + alt_decimal_num;
    println!("The sum is {sum}");

    // boolean data types
    let is_rust = true;
    println!("Is he a Rust? {is_rust} he is Rust.");

    // character primitive data types in Rust
    let c = 'z';
    let d = '5';
    println!("{c} {d}");

    // compound types in Rust

    // Tuples
    let my_tup = (30.3, 'b', "rust", true);
    println!("My tuple is {:?}", my_tup);

    // A wrap of mutable tuples coz they are mmutable by default
    let mut tup = ("ken", 23.3, true, 'a', "rust");
    println!("tup.0 = {:?}.", tup.0);
    tup.4 = "What in the world is this??";
    println!("tup.4 = {:?}. ", tup );

    // Arrays

    // Arrays in Rust
    let my_array = [0, 1, 2, 3, 4];
    /* * Arrays only accepts a single type unlike tuples or enum so this declaration will result into conflicts [0, 1, 2, 3, 4, 5, 'a', "rust"]
    * */
    println!("My array is {:?}", my_array); // printing the array my_array

    // A wap of variables in Rust. It was great learning variables in Rusts...
}
