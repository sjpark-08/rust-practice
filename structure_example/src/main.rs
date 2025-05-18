#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width = 20;
    // let height = 30;
    // let rect = (20, 30);

    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    
    // println!("The area of the rectangle is {} square pixels.", area(width, height));
    // println!("The area of the rectangle is {} square pixels.", area(rect));
    println!("The area of the rectangle is {} square pixels.", area(&rect));
    println!("rectangle = {:?}", rect);
    dbg!(rect);
}

fn area(rect: &Rectangle) -> u32 {
    // width * height
    // rect.0 * rect.1
    rect.width * rect.height
}
