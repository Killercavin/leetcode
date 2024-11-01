// Functions in Rust
/* fn salary(){
    let monthly_salary = 150.00;
    let daily_salary;
    daily_salary = monthly_salary / 30.00;
    let rating_salary_per_hour = daily_salary / 24.00;

    // returning the function
    return println!("The monthly salary is {monthly_salary}$.\nThe daily salary is {daily_salary}$.\nThe hourly salary is {rating_salary_per_hour}$.");
} */

fn another_func(x: i32){
    return println!("The value of x is {x}");
}

fn main() {
    println!("This is functions in Rust!");

    another_func(5);
}
