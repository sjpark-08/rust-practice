// enum Result <T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    // let file = File::open("hello.txt");
    // 
    // match file {
    //     Ok(f) => println!("successfully opened file"),
    //     Err(e) => panic!("error opening file")
    // }
    
    let file = File::open("test.txt");
    let file = match file {
        Ok(f) => Ok(f),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => File::create("test.txt"),
            _ => panic!("access failed")
        }
    };
    
    let username = read_username();
    println!("{:?}", username);
}

fn read_username() -> Result<String, Error> {
    let mut file = File::open("test.txt")?;
    
    let mut username = String::new();
    
    file.read_to_string(&mut username)?;
    Ok(username)
}
