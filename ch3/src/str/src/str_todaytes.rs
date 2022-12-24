fn main(){
    let pr = "猫に小判";
    for c in pr.bytes(){
     println!("{:2x}",c);
    }
    println!("\nバイト数={}B",pr.len());
}