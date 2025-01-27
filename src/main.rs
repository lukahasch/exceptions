use exceptions::{catch, throw};

fn rerror() {
    throw(String::from("test"));
}

fn main() {
    let x = catch!({
        rerror();
        0
    } except e: String {
        eprintln!("caught error: {}", e);
        20
    });
    dbg!(x);
}
