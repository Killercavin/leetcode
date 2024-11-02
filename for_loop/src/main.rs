fn main() {
    let a = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90];
    for i in a{
        println!("The element is {}", i);
    }

    for number in (0..10).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}