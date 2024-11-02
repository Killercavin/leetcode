/* fn main() {
    /* loop{
        println!("Loop again!");
    } */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
 */

fn main(){
    // loop
    let mut counter = 0;
    let _my_loop = loop { // used var declaration _my_loop to prevent the "unused variable" warning
        counter += 1;

        if counter == 25{
            break println!("The counter is equal to {}", counter);
        }
    };
}