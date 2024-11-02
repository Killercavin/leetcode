/* fn main() {
    let mut counter = 0;
    let state: bool = true;
    while state{
        if counter == 25{
            println!("The counter is {}", counter);
        }
        counter += 5;
    }
}
 */

fn main(){
    let mut number: i128 = 25000000000000000000000000000000000000; // 25 decilion fine impossed on Google by Russian court!
    
    while number >= 0{
        println!("{number}");
        number -= 1;
    }
}