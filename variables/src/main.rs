fn main() {
    let x: i32 = 12;

    let x = x + 1;

    {
        let x = x * 2;
        println!("{x}")
    }

    println!("{x}")
}
