#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    // println!("The area of the rectangle is {} square pixels.", rect.area());
    println!("rectangle = {:?}", rect);
    dbg!(rect);
    println!("The area of the square is {:?} square pixels.", Rectangle::square(20));
}
