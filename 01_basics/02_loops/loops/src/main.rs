#![allow(dead_code)]


fn main() {
    // loop_to10();
    // for_to10();
    array_loop();
}

fn loop_to10() {
    let mut n = 0;

    loop {
        n += 1;
        println!("Hello World!");
        if n > 10 {
            return
        }
    }
}

fn for_to10() {
    for _n in 0..10 {
        println!("Hello World!")
    }
}

fn array_loop() {
    // let mut v = Vec::new();
    // v.push(4);
    // v.push(8);
    // v.push(12);
    let mut v = vec![1,2,3,4,5];
    v.push(6);

    for n in v {
        println!("{} ", n)
    }
}
