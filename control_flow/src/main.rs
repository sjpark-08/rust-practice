fn main() {
    let x = 3;
    if x % 3 == 0 {
        println!("x is divisible by 3");
    } else if x % 3 == 1 {
        println!("x division by 3 is 1");
    } else {
        println!("x division by 3 is 2");
    }
    
    let condition = true;
    
    let y = if condition { 5 } else { 6 };
    println!("y = {}", y);
    
    let mut counter = 0;
    loop {
        counter += 1;
        println!("iter");
        if counter == 3 {
            break;
        }
    }
    
    let xs = [1, 2, 3, 4, 5];
    let mut idx = 0;
    while idx < xs.len() {
        println!("xs[{idx}] = {}", xs[idx]);
        idx += 1;
    }
    
    for x in xs {
        println!("x = {}", x);
    }
    
    for i in (0..idx).rev(){
        println!("i = {}", i);
    }
}
