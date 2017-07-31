//! # モジュールドキュメント
//!
//! このライブラリは....

/// # ドキュメンテーションコメント
/// ## markdownをサポート
/// ```
/// do_something();
/// ```
fn main(){
    /*
    println!("Hello, world!");
    let x = 5;
    print_x(x);
    let y = add_one(x);
    println!("add_one:{}", y);
    let f: fn(i32) -> i32 = add_one;
    println!("f():{}", f(4));
    check_bool();
    check_char();
    check_array();
    check_tuple();
    */
    check_if();
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

fn check_char() {
    let x = '💕';
    println!("{}", x);
}

fn check_array() {
    //let a = [1, 2, 3];
    let mut m = [1, 2, 3];
    m[1] = 10;
    println!("{}",m.len());
}

fn check_tuple(){
    let  tuple = (1,2,3);
    println!("{}",tuple.0);
}


fn check_if() {
    let x = 5;
    if x == 5 {
        println!("aaa");
    } else if x == 6 {
        println!("bbb");
    }
    let y = if x == 5 {10}else{32};
    println!("{}",y);
}


