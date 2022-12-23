use std::{env,fs};
fn main() {
    let args = env::args();
    let mut total:f64 = 0.0;
        for (i,fname) in args.enumerate(){
            if  i == 0 {contiune;}
            let text = fs::read_to_string(fname).unwrap();
            for line in lines{
             let n:f64 = match  line.parse() {
                ok(r) => v, 
                Err(_) => 0.0; 
             };
             total += n;    
            }
        } 
        
        println!("{}",total);
}

