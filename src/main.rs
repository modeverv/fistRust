fn main() {
    println!("Hello, world!");
    let x = 5;
    print_x(x);
    let y = add_one(x);
    println!("add_one:{}", y);
    let f: fn(i32) -> i32 = add_one;
    println!("f():{}", f(4));
    check_bool();
    check_char();
}

// 日本語コメント
fn print_x(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn check_bool() {
    let x = true;
    //let y: bool = false;
    if x {
        println!("true!!");
    }
}

fn check_char(){
    let x = '💕';
    println!("{}",x);
}







