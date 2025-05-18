fn smallest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}

// struct generic
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

fn main() {
    let numbers = vec![3, 1, 3];
    let chars = vec!['b', 'a', 'c'];

    println!("The largest number is {}", smallest(&numbers));
    println!("The smallest char is {}", smallest(&chars));

    let result = smallest(&["홍길동", "둘리", "도우너"]);
    println!("The smallest result is {}", result);

    let p3 = Point { x: 2, y: 3 };
    println!("p3 = {:?}", p3);
}
