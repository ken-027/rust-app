fn casting() -> i32 {
    let number: i32 = "-23".parse().expect("Not a number");
    return number;
}

fn decimal() -> f32 {
    // return "32.5".parse().expect("Not a number!");
    return "32".parse().expect("Not a decimal");
}

fn tuple() -> (i32, i32, i32) {
    return (40, -3, 32);
}

fn numbers() -> [u32; 4] {
    return [2, 3, 56, 6];
}

// fn first_quarter() -> [str; 3] {
//     return ["January", "February", "March"];
// }

pub fn main() {
    let casting = casting();
    let decimal = decimal();
    let character: char = 'K';
    let (_x, _y, _z) = tuple();
    let num_1 = numbers()[0];
    let first_quarter: [&str; 3] = ["January", "February", "March"];

    println!("Casting number {casting}");

    println!("Decimal number {decimal}");

    println!("Character {character}");

    println!("Tuple {_x}");

    println!("1st index of numbers is {num_1}");

    let january: &str = first_quarter[0];
    println!("Month start in {january}");
}
