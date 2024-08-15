// fn main() {
//     println!("Hello, world!");

//     another_function(5, 'h');
// }

// fn another_function(value: i32, unit_label: char) {
//     println!("Another function.");
//     println!("The value of value is: {value}{unit_label}");
// }

fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("the value of y is: {y}");
    let f = five();
    println!("The value of f is: {f}");
}

fn five() -> i32 {
    5
}
