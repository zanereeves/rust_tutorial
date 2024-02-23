fn main() {
    let x = 5;
    println!("The val of x is: {x}");
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope: {x}");
    }

    println!("The value of x in outer scope is: {x}");
}
