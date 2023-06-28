use learn_rust::spaces;

pub fn convert_fah_cel(fahrenheit: f64) -> f64 {
    ((fahrenheit - 32.0) * 5.0) / 9.0
}

pub fn fibonacci(terms: usize) -> Vec<u32> {
    let mut items = vec![0, 1];
    let mut index = 2;
    while index <= terms {
        items.push(items[index - 2] + items[index - 1]);
        index += 1;
    }

    items
}

pub fn main() {
    let fahrenheit = 120.0;

    spaces(1);
    println!("1. Convert temperatures between Fahrenheit and Celsius.");
    println!(
        "The celsius of fahrenheit {fahrenheit} is {:.3}",
        convert_fah_cel(fahrenheit)
    );

    spaces(1);
    println!("2. Generate the nth Fibonacci number.");
    for item in fibonacci(10) {
        print!("{item} ");
    }

    // TODO: here
    spaces(2);
    println!("3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.");
}
