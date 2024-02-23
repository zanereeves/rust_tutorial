fn main() {
    let s = "hello";

    // This string type manages data stored on the heap
    let mut s1 = String::from("hello");

    s1.push_str(", world!");
    println!("{}", s1);

    // Below an error is thrown as the rust considers s2 invalid to avoid double free from heap
    // let s2 = String::from("hello Vanier!")
    // let s3 = s2;
    // println!("{}", s2);


    // The below works though

    let s2 = String::from("hello");
    let s3 = s2.clone();

    println!("s2 = {}, s3 = {}", s2, s3);

    // Important to note that this behavior is only relevant to variables stored on the heap.
    //

    takes_ownership(s2);
    // If we were to call s2 within the current scope now that the function
    // takes_ownership has claimed s2 an error will be thrown.
    //

    let s4 = gives_ownership();

    let s5 = takes_and_gives(s4);

    println!("{}", s5);



    let mut test_string = String::from("hello");
    let len = calculate_len(&test_string);


    println!("The length of: {} is: {}", test_string, len);

    change(&mut test_string);

    println!("{}", test_string);


    // All of these features help prevent dangling points in rust
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn calculate_len(s : &String) -> usize {
    s.len()
}

fn gives_ownership() -> String {

    let some_s = String::from("given, taken, then given");

    some_s
}

fn takes_and_gives(s : String) -> String {
    s
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_int: i32) {
    println!("{}", some_int);
}

