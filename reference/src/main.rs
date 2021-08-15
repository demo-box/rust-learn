fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("The changed string is '{}'", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(ss: &mut String) {
    ss.push_str(",world");
}
