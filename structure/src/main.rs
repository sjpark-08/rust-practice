struct User {
    name: String,
    email: String,
    active: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn build_user(name: String, email: String) -> User {
    User {
        name: name,
        email: email,
        active: true
    }
}

fn main() {
    let mut user = User {
        name: String::from("홍길동"),
        email: String::from("gildong@example.com"),
        active: true,
    };
    
    user.email = String::from("gd.hong@example.com");
    println!("이용자의 이름은 = {}", user.name);
    println!("email = {}", user.email);
    
    let mut user2 = build_user(String::from("abc"), String::from("abc"));
    println!("email = {}", user2.email);
    
    let user1 = User {
        name: String::from("ab"),
        email: String::from("ab@abc.com"),
        active: true
    };
    
    let user3 = User {
        active: false,
        ..user1
    };

    let color = Color(1, 2, 3);
    let point = Point(1, 2, 3);
    color.0;
}