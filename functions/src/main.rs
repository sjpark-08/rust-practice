fn main() {
    println!("Hello, world!");

    a_function();

    print_number(5);

    add_numbers(3, 5);
    
    let a = circle_area(2.0);
    println!("a = {}", a);
}

fn a_function() {
    println!("another function");
}

fn print_number(x: i32) {
    println!("x = {}", x);
}

fn add_numbers(x: i32, y: i32) {
    let sum = x + y;
    println!("x + y = {}", sum);
}

fn circle_area(radius: f64) -> f64 {
    const PI: f64 = 3.14;
    
    PI * radius * radius
}