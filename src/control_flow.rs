use learn_rust::spaces;

fn is_senior(num: u32) -> bool {
    !(num <= 60 && num >= 1)
}

fn looping(mut repeat: u32) {
    let _result = loop {
        repeat -= 1;

        println!("{repeat}");

        if repeat <= 0 {
            break 2 * 2;
        }
    };

    println!("Result is {_result}");
}

fn nested_loop() {
    let mut count: u32 = 0;
    'counting_up: loop {
        println!("count: {count}");

        let mut remaining: u32 = 10;

        loop {
            println!("remaining: {remaining}");

            let _ = remaining == 9 && break;

            let _ = count == 2 && break 'counting_up;

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn while_loop(mut index: u32) {
    println!("count down...");
    while index != 0 {
        println!("{index}");

        index -= 1;
    }

    println!("Kaboom!");
}

fn for_loop(collection: [u32; 5]) {
    for item in collection {
        println!("{item}");
    }
}

fn for_loop_rev() {
    for num in (1..5).rev() {
        println!("{num}");
    }
}

pub fn main() {
    let age = 32;
    let addressing = if is_senior(age) { "" } else { "not" };

    println!("Age {age} is {addressing} a senior citizen");
    spaces(1);

    looping(1);

    spaces(1);
    nested_loop();
    spaces(1);
    while_loop(5);

    spaces(1);
    println!("For loop...");

    for_loop([23, 56, 2, 3, 5]);

    spaces(1);
    println!("For loop reverse...");

    for_loop_rev();
}
