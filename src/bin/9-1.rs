// Unrecoverable Errors with panic!
fn main() {
    let arr = vec![1, 2, 3];
    arr[10]; // panic
             // RUST_BACKTRACE=full cargo run --bin 9-1
             // for detail
}
