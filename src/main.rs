fn main() {
    for i in 1..3 {
        println!("Hello, world! {}", invert(i));

    }
}

fn invert(i: i32) -> i32 {
    return -i
}


fn invert2(i: f64) -> f64 {
    return -i
}