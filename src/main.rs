fn main() {
    let x = {
        let y = 0;
        y + return_value() * 1
    };

    println!("Hello, world! {}", x);
}

fn return_value() -> i32 {
    56
}
