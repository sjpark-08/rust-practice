struct ImportantPart<'a> {
    part: &'a str,
}

fn lifetime_in_struct() {
    let sentences = String::from("안녕하세요. 러스트의 참조 수명에 대해 알아볼게요.");
    let first_sentence = sentences
        .split('.')
        .next()
        .expect("마침표 '.'를 찾을 수 없어요");
    let i = ImportantPart {
        part: first_sentence,
    };
}

fn main() {
    let s1 = String::from("가나다");
    let s2 = "하나둘셋";
    
    let res = longest(s1.as_str(), s2);
    println!("{}", res);
    
    let s: &'static str = "프로그램 실행 중 내내 유효한 수명";
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}