pub fn constant() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("3 hours in seconds: {THREE_HOURS_IN_SECONDS}");
}

pub fn shadowing() -> u32 {
    let x: u32 = 5;

    let x: u32 = x + 32;

    return x;
}

pub fn main() {
    println!("Learning Rust Language!\n\n");

    constant();

    let shadow_var: usize = shadowing().try_into().unwrap();
    println!("Shadowed variable {shadow_var}");
}
