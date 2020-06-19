fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn increase_by(mut val: u32, how_much: u32) {
    val += how_much;
    println!("You made {} points", val);
}

fn main() {
    let a: u64 = 17;
    let b = 3;
    let result = add(a, b);
    println!("Result {}", result);

    let score = 2048;
    increase_by(score, 30);
}
