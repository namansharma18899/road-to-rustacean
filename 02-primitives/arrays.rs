/*
Pretty much the same but here we can borrow a slice of an array ??
0 indexed
Slice -> point to section of array, form [starting_index..ending_index]
 */

fn main(){

    // declaring my first array
    let arr:[i32; 5] = [1,2,3,4,5];
    println!("Array -> {}", arr[0]);
    println!("len -> {}", arr.len());
    for index in 0..arr.len(){
        println!(" INdex -> {}  val -> {}", index, arr[index]);
    }
    /*
    slices
     */
    // let slice = arr[0; 3];
    let slice = &arr[1..3];
    println!("slice el -> {}", slice[0]);

    for i in 0..arr.len() + 1 { // Oops, one element too far!
        match arr.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

}