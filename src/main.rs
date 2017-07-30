


fn main() {
    println!("Hello, world!");
    let x = 5;
    print_x(x);
    let y = add_one(x);
    println!("add_one:{}",y);
}


fn print_x(x: i32){
    println!("x is: {}",x);
}

fn add_one(x:i32) -> i32 {
    x + 1
}










