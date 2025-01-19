use exceptions::{catch, throw};

fn rerror() {}

fn main() {
    let x = catch!({
        rerror();
        0
    } except e: &str {
        eprintln!("caught error: {}", e);
        10
    });
    dbg!(x);
}
