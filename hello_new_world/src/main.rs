use std::io;

fn main(){
    let _variable: i32=10;
    let mut _some_other_var: String = String::new();
    println!("Enter the number");
    io::stdin().read_line(&mut _some_other_var).expect("failed to read lines sen~or");
    println!("You guessed -> {_some_other_var}");
    println!("1unsigned -2 Hmm  = {}", 1u32 - 2u32);
    // Todo: whhy is if not working ??
    if (_variable.to_string().eq(&_some_other_var))== true{
        println!("Correcto");
        println!("{_variable} {_some_other_var}");
    }else {
        println!("No correcto");
        println!("{_variable} {_some_other_var}");
    }
}