fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(10);

    let y = {
        let x = 3;
        x + 1
    };

    let fives = five();
    println!("The value of y is: {y}");
    println!("The value of fives is: {fives}");
}

fn five() -> i32 {
    5
}

fn another_function() {
    println!("Another function!");
}

fn another_function2(x: u32) {
    println!("The value of inputted number is: {x}");
}
