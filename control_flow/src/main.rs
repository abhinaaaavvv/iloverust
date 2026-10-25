fn main() {
    let x: i32 = 5;

    let y: i32 = if x > 0 { x } else { 1 };
    println!("{y}");

    let mut counter = 1;
    let result = loop {
        counter *= 2;

        if counter >= 1000 {
            break counter;
        }
    };
    println!("{result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
