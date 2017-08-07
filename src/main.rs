//! # モジュールドキュメント
//!
//! このライブラリは....

/// # ドキュメンテーションコメント
/// ## markdownをサポート
/// ```
/// do_something();
/// ```
fn main(){

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
    check_if();
    check_loop_while_for();
    check_foo();
    check_foo2();
    check_mut();
    check_lifetime();
    check_mut2();
    check_struct();
    check_enum1()
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

fn check_loop_while_for() {
    let mut check = true;
    let mut i = 0;
    loop {
        i = i + 1;
        if i > 3 { break; }
    }
    while check {
        i = i + 1;
        if i > 10 { check = false; }
    }
    for x in 0..10 {
        println!("{}",x);
    }
    for (i,j) in (5..10).enumerate() {
        println!("i = {}, j = {}",i,j);
    }
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer;} // x ループを継続
            if y % 2 == 0 { continue 'inner;} // y ループを継続
            println!("x:{},y:{}",x,y);
        }
    }
}

/// ＃退屈なRust
/// こんなRustはいやだ
fn check_foo() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let (v1, v2, answer) = foo(v1, v2);
    println!("{}", answer);
}

fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    (v1, v2, 42)
}

/// #借用
/// 借用では束縛はimmutable
fn check_foo2() {
    let v1 = vec![1, 2, 3];
    let answer = foo2(&v1);
    println!("{}", answer);
}

fn foo2(v: &Vec<i32>) -> i32 {
    //v.push(3) // これは出来ない
    42
}

/// #&mut参照
fn check_mut() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);
}

/// #ライフタイム
struct Foo<'a> {
    x: &'a i32,
}
fn check_lifetime(){
    let y = &5;
    let f = Foo{x:y};
    println!("{}",f.x);
}

use std::sync::Arc;
use std::cell::RefCell;

/// exterior mutabillity
/// interior mutabillity
fn check_mut2() {
    let mut x = 5;
    {
        let mut y = &mut x;
    }
    println!("{}", x);

    // exterior
    let x1 = Arc::new(5);
    let y1 = x1.clone();
    // interior
    let x2 = RefCell::new(3);
    let y2 = x2.borrow_mut();
    //....わからん
}

struct Point {
    x: i32,
    y: i32,
}
// タプル構造体
struct Color(i32,i32,i32);
struct Inches(i32);

fn check_struct() {
    let mut point = Point { x: 0, y: 0 };
    point.x = 5;
    println!("{},{}", point.x, point.y);
    let point = point; // ここから束縛はここから変更不可

    let mut p2 = Point { y: 1, ..point };
    println!("{},{}", p2.x, p2.y);

    let black = Color(0,0,0);
    //newtypeパターン
    let length = Inches(10);
    let Inches(len) = length;
    println!("{}",len);
}

// enum

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn check_enum1() {
    let m = Message::Write("Hello world".to_string());
    let v = vec!["Hello".to_string(),"World".to_string()];
    let v1:Vec<Message> = v.into_iter().map(Message::Write).collect();
}