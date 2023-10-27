use std::io::{self, BufRead};
use std::{process, env};
use std::result::Result;
fn validate_file(file: &str) -> Result< &str,&'static str >{
   if file == "test"{
        return Ok("200");
    }else {
        println!("Error: File not found");
        return Err("ERROR NOT FOUND");
    }
}
fn get_guests(string: &str) -> Option<&str> {
    let words: Vec<&str> = string.split_whitespace().collect();

    for(i,word) in words.iter().enumerate(){
        if word.to_string() == "greet" && i+1 < words.len() {
            return Some(words[i + 1]);
        }
    }
    return None;
}

fn assess_line(line:String){
    if line == "exit" {
        println!("COMMAND EXITED");
        process::exit(0);
    }
    if line.contains("greet") {
        match get_guests(line.as_str()){
            Some(str)=> {
                println!("Hello {}!! ", str );
            },
            None => {
            println!("Who do I greet though?");
            }
        }
    }
    if line.contains("morning") {
        println!("Good Morning! ")
    }
    if line.contains("introduce"){
        println!("I'm Dumb interpreter. It's nice to see you! ")
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                assess_line(line.unwrap());
            }
        },
        2 => {
            match validate_file(std::env::args().nth(1).expect("no pattern given").as_str()){
                Ok(str) =>  {
                    println!("{}",str);
                }, 
                Err(str) => {
                    println!("{}",str);
                } 
            }
        },
        _ => {
            println!("I don't know what the fuck you are doing")
        }
    }
}
