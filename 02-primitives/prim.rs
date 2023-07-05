fn main(){
    let tupple = (12u32, -12i32, "naman", 's', true);
    println!("typple -> {}", tupple.1);

    let (val1, val2) = (tupple.0, tupple.1);
    println!("val 1- > {}, val 2 -> {}", val1, val2);
}