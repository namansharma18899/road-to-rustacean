use std::io;

fn main(){
    let _variable=10;
    let mut _some_other_var = String::new();
    println!("Enter the number");
    io::stdin().read_line(&mut _some_other_var);
    println!("number entered -> {}", _some_other_var);
}