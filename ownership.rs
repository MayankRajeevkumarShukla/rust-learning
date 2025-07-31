fn main(){
    // let s ="hello";
    let mut s =String::from("hello");
    s.push_str(", World");
    println!("{s}");
    let x =5;
    let y =x;
       let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1},world")
    // the above line does not work bcoz rust doens not consider this a part anymore and to not cause any error of doublcleaning this is done by the rust so baiscally s1 moves to s2 is what happend
    let mut sl = String::from("hello");
    sl = String::from("ahoy");

    println!("{sl}, world!");
    // ahoy world bcoz the first sl sting goes out of the scope bcoz it is no longer considerd

    let s11 = String::from("hello");
    let s22 = s11.clone();

    println!("s11 = {s11}, s22 = {s22}");
    // so here it will clone and have a differnt space in the heap memory

    // remember we can copy a number bcoz it a small scallar value and it can be done quick on the heap memory

}