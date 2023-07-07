/*

Shadowing is basically overwriting same variable
 */

fn main(){
    let spaces = "naman";
    println!(" value -> {}", spaces);
    let spaces = spaces.len();
    println!(" value post shadoing -> {}", spaces);
}