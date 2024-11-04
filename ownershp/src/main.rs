fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; //.clone();
    println!("{}, world!", s1);
    // println!("Is s1 equal to s2? {}", s1 == s2); // This will not compile when clone is not used
}
