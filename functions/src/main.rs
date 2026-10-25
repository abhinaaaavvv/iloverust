fn main() {
    hello_world();
    println!("{}", add(12, 23));

    let y = {
        let x = 5;
        x + 2
    };
    println!("{y}")
}

fn hello_world() {
    println!("hello world!!!")
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
