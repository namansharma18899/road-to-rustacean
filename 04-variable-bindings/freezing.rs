fn main(){
    let mut test_variable;
    fn test(){
        let test_variable = 11;
        println!("tv -> {}", test_variable);
    }
    test();
    test_variable = 12;
    println!("tv post fn scope {}", test_variable);
}