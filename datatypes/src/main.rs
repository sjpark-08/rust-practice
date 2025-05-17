fn main() {
    // tuple
    let t: (i32, bool, f64) = (32, true, 1.41);

    let (x, y, z) = t;
    println!("{}, {}, {}", x, y, z);

    // array
    let arr =  [1, 2, 3, 4, 5];
    println!("arr[3] = {}", arr[3]);

    let threes = [3; 100];
    let last = threes[99];
    println!("last = {last}");

    let hellos = ["hello"; 10];
    println!("{:?}", hellos);
}
