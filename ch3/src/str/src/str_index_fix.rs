fn main(){
    let s = "こんにちは";
    let ch = s.chars().nth(0).unwrap();
    println!("{}",ch);
    let ch2 = s.chars().nth(2).unwrap();
    println!("{}",ch2);
}