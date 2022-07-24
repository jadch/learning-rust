fn panic_function() {
    // For any panicking program, cargo run with this flag to see a backtrace: "RUST_BACKTRACE=1 cargo run"
    panic!("this is me throwing an error")
}

fn main() {
    panic_function();
    println!("Hello, world!");
}
