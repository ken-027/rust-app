fn sum(num1: u32, num2: u32) -> u32 {
    return num1 + num2;
}

fn make_0(_num: &mut u32) {
    // _text = "";
    *_num = 0;
}

fn make_empty(_text: &mut &str) {
    *_text = "";
}

pub fn main() {
    let num1 = 32;
    let num2 = 21;
    let mut number = 23;
    let mut name = "Ken";
    let total: u32 = sum(num1, num2);

    println!("Total sum of {total}");

    println!("Number before {number}");
    make_0(&mut number);
    println!("Number after {number}");

    println!("Name before {name}");
    make_empty(&mut name);
    println!("Name after {name}");
}
