fn main() {
    let mut x = 3;
    println!("x = {x}");
    x = 7;
    println!("x = {x}");

    const PI: f32 = 3.14592;
    println!("PI = {PI}");

    {
        // shadowing
        let x = x * 2;
        println!("x = {x}");
    }
}
