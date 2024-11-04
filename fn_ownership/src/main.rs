fn take_ownership(some_string: String){
    return println!("Some string is: {}", some_string);
}

fn make_copy(some_integer: i32){
    return println!("Some integer is: {some_integer}");
}

fn main() {
    let s = String::from("Hello"); // s comes into scope

    /* The value of s is moved into the function making it invalid on this scope again --> the drop has been called for this scope and the memory deallocation is archieved freeing s from using system memory
     */

    // calling the functions
    take_ownership(s);

    // println!("{}", s); // This won't work out coz s is already out of scope!!!

    let x = 555; // x gets into scope

    make_copy(x);

    // println!("{}", x); //This will work out coz x is a copy and still in scope at this point

    println!("Like what I see, Rust is amazing!");
}
