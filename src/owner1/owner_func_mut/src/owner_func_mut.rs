fn add_quote(msg: &mut String){
    msg.insert(0,'『');
    msg.push('』');
}

fn main() {
    let mut msg = String::from("強い心は病気の支えになる");
    println!("{}",msg);
    add_quote(&mut msg);
    println!("{}",msg);
}
