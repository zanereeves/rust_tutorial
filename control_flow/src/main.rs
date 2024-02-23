fn main() {
    let number = 3;

    if number < 5 {
        println!("cond is true");
    } else {
        println!("cond is false");
    }

    let cond = false;

    let num = if cond {5} else {10};

    println!("The val of num is: {num}");

    let mut counter = 0;

    let res = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {res}");


    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;


        loop {
            println!("remaining = {remaining}");


            if remaining == 8 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;

    }

    println!("End outer_count is: {count}");


    let a = [100, 200, 300, 400];


    for ele in a {
        println!("The val of ele: {ele}");
    }



    let mut n1 : i64 = 0;
    let mut n2 : i64 = 1;

    for _ in 1..100 {
        let tmp = n1 + n2;
        n1 = n2;
        n2 = tmp;
    }

    println!("The 100th fib number is: {n1}");
}
