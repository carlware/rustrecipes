

fn main() {
    let b = highest(2,3,10);

    println!("{} is highest", b);
}

fn highest(a:i32, b:u32, c:i8) -> i32 {
    let mut res = a;
    if b as i32 > res {
        res = b as i32;
    }
    if c as i32 > res {
        res = c as i32
    }
    res
}
