
fn main(){
    struct Person {
        age: i32,
        name: String,
    }
    /*
    Ezy Pzy stuff
     */
    struct PersonTuple(i32, i32);
    let  name= String::from("naman");
    let age = 32;
    let mut ptupple = PersonTuple (21,44);
    ptupple.0 = 45;
    println!("{:?}", ptupple.0);
    let mut aboy = Person {age, name};
    aboy.name = String::from("Tiwatiya");
    println!("{:?}", aboy.name);

    /*
    Multi Struct Alchemy 
     */

    struct Point {
        x: i32,
        y: i32,
    }
    struct circle(Point, Point, Point);
    let smallPoint = Point {x:12, y:33};
    let arr:[Point; 3] = [Point {x:12,y:33}, Point {x:11,y: 44}, Point {x:12,y:00}];
    println!(" arr -> {}", arr[0].x);
}