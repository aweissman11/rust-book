fn main() {
    // let mut x = 5;
    // println!("the value of x is: {x}");
    // x = 6;
    // println!("the value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("the value of x is: {x}");

    // let y = 5 / 3; // <== Doesn't work. Just spits out an integer
    let y: f64 = 5.0 / 3.0;
    println!("the value of y is: {y}");

    // tuple (fixed length, mixed types)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (xx, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // array (fixed length, single type)
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
