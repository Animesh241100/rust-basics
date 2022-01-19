fn test_match_statement() {
    println!("****************Test Match statement***************\n");
    
    // with single variable
    let x = 6;
    match x%3 {
        // cases the match variable can have
        1 => println!("Remainder is 1"),    
        2 => println!("Remainder is 2"),
        _ => println!("Divisible by 3")  // default case
    }


    // for taking all possible cases into consideration
    let is_empty = true;
    let is_closed = true;
    match (is_empty, is_closed) {
        (false, false) => println! ("is not empty and is not closed"),
        (false, true) => println! ("is not empty and is closed"),
        (true, false) => println! ("is empty and is not closed"),
        _ => println! ("impossible case!!!"),
    }
}


fn test_iterators() {
    println!("****************Test Iterators***************\n");
    let mut i = 1;
    
    // infinite loops
    loop {
        if i > 1000 {
            break;
        }
        println!("i: {}", i);
        i *= 2;
    }

    // while loops
    let mut j = 2;
    while j < 1000 {
        println!("j: {}", j);
        j *= 2;
    }

    // for loops
    for x in 0..20 {         // [0,20) right-exclusive range
        println!("hello : {}", x);
    }

    for x in 0..=20 {        // [0,20] inclusive range
        println!("hello again: {}", x);
    }

    let arr_a = ["text", "good", "bad", "minus"];
    for val in arr_a {
        println!("arr value: {}", val);
    }
}


fn test_conditionals() {
    println!("****************Test Conditionals***************\n");
    let num = 2;
    if num == 1 {
        println!("You won");
    } else if num == 2 {
        println!("Try again");
    } else {
        println!("You lost");
    }
}




fn test_composite_types() {
    println!("****************Test Composite Types***************\n");

    // Initializig a tuple
    let tup_a = ("form", true, 45, 6.7);   // all tuple elements can be different data types
    println!("first: {}, second: {}", tup_a.0, tup_a.1);

    let (word, _, _, height) = tup_a;
    println!("word: {}, height: {}", word, height);

    // Initializing an array
    let arr1 = [12,13,15,16];         // all array elements have to be of same type
    println!("2nd element: {}", arr1[2]);

    const ARR_SIZE:usize = 15;
    let value:i8 = -1;
    let arr2 = [value;  ARR_SIZE];         
    println!("arr2: {:?}", arr2);

    let mut arr3: [i8; ARR_SIZE] = [0; ARR_SIZE];
    println!("arr3: {:?}", arr3);

    arr3[1] = 10;
    println!("updated arr3: {:?}", arr3);  // format string for debugging 

}


// rust is statically typed, i.e. var types can't be changed once fixed at the declaration
fn test_simple_types() {
    println!("****************Test Simple Types***************\n");
    let num:i8 = 127;   // signed 8 bit
    let u_num:u8 = 128;   // unsigned 8 bit
    let word:&str = "my word";
    println!("num: {}, word: {}, u_num: {}", num, word, u_num);

    let sym_a = 'a';   // utf8 or ascii
    let sym_b = '$';
    println!("sym_a: {}, sym_b: {}", sym_a, sym_b);

    let is_empty = false;
    if is_empty {
        println!("List is empty");
    }
    else {
        println!("List is not empty");
    }
}


fn main() {
    println!("Hello, world!");
    let mut num = 10;        // mutable variable
    println!("num: {}", num);
    num = 100;
    println!("new num: {}", num);
    let word = "animesh";    // immutable variable
    println!("name: {}, num: {}", word, num);

    test_simple_types();
    test_composite_types();
    test_conditionals();
    test_iterators();
    test_match_statement();
}
