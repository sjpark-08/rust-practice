fn main() {
    let mut s: String = String::from("hello");
    println!("s = {}", s);
    s.push_str(", world!");
    println!("s = {}", s);

    {
        let x = 3;
        let y = x;
        println!("x = {}, y = {}", x, y);

        let s1 = String::from("hello");
        println!("s1 = {s1}");
        let s2 = s1;
        println!("s2 = {}", s2);
        let s3 = s2.clone();
        println!("s2 = {}, s3 = {}", s2, s3);
    }



    let s = String::from("hello");
    // string_length(s); // 소유권 이동
    // println!("s = {}", s); <- 불가능

    // primitive type은 스택에 저장 -> 값 복사
    let x = 3;
    double(x);
    println!("x = {}", x);

    // 반환값 소유권 이동
    let s = return_string_length(s);
    println!("s = {}", s);

    let (len, s) = get_string_length(s);
    println!("s_len : {}", len);
    println!("s = {}", s);

    // borrowing
    let len = calc_length(&s);
    println!("len = {} s = {}", len, s);
    
    let s = String::from("hello world!");
    
    let word = &s[0..6];
    let word = &s[6..s.len()];
    println!("word = {}", word);
    println!("first word = {}", first_word(&s));
    
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    
    println!("a = {:?} slice = {:?}", a, slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn string_length(s: String) {
    println!("length = {}", s.len());
}

fn double(x: i32) {
    println!("x = {}", x);
}

fn return_string_length(s: String) -> String {
    println!("string_length = {}", s.len());
    s
}

fn get_string_length(s: String) -> (usize, String) {
    println!("s = {}", s);
    (s.len(), s)
}

fn calc_length(s: &String) -> usize {
    let length = s.len();
    length
}