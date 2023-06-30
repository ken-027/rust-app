use crate::helpers::spaces;

fn variable_scope() -> String {
    let mut s = String::from("value");
    s.push_str(", value 2");
    let s1 = s.clone();
    s1
}

fn number_scope() -> (u32, u32) {
    let x = 11;
    let y = x;
    (x, y)
}

fn modify_argument(_text: &mut String) {
    _text.push_str(" append text");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn main() {
    println!("Variable Scope!");
    println!("{}", variable_scope());

    spaces(1);
    println!("Number Scope!");
    let (x, y) = number_scope();
    println!("x: {}, y: {}", x, y);

    spaces(1);
    println!("Reference Scope!");
    let mut _str = String::from("hello rustaceans");
    modify_argument(&mut _str);
    println!("{}", _str);

    spaces(1);
    println!("Borrow value!");
    let mut text = String::from("Hello");
    let val1 = &text;
    // {
    //   let val2 = &mut text;
    // }
    let val2 = &text;
    println!("{}, {}", val1, val2);
    let val3 = &mut text;
    println!("{}", val3);
}
