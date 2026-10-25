// scalar ; number, floating point, boolean, characters
// compound: tuples, arrays
fn main() {
    //scalar
    let num: i32 = "123".parse().expect("no a number");
    println!("{num}");

    let dec = 12.1; //f64
    let decc: f32 = dec;
    println!("{decc}");

    let f: bool = true;
    let c = 'z';
    println!("{f} {c}");

    //compound
    // tuples - fixed size, collection of diff data types
    let tup: (i32, u32, bool) = (12, 23, true);
    let (a, b, c) = (1, 2, 3);
    let (x, y, z) = tup;

    let ele = tup.1;
    let ele2 = tup.2;
    println!("{a} {b} {c} {x} {y} {z} {ele} {ele2}");

    //arrays - fixed size, collection of same data types
    let arr = [1, 2, 3, 4, 5];
    let days = [
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];
    let a = [3; 5];

    let first = arr[0];
    println!("{first}");

    for ele in arr {
        println!("{ele}");
    }

    for day in days {
        println!("{day}");
    }

    for ele in a {
        println!("{ele}")
    }
}
