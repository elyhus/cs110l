use std::env;
use std::process;
use std::fs::File;
use std::io::{self,BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let (mut lc,mut wc,mut cc)=(0,0,0);
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    for line in io::BufReader::new(file).lines(){
        let lines=line.unwrap();
        wc+=lines.split_whitespace().count();
        cc+=lines.chars().count();
        lc+=1;
}    
    println!("lines:{},word:{},char:{}",lc,wc,cc);
    // Your code here :)
}
