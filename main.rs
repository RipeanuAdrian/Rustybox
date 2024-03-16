/* Suggestions: 1. Write a function for every command
                2. Start with the pwd command
                3. Continue with the other commands that do not have parameters
*/
use std::env;
use std::fs::File;
use std::io::Read;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

fn pwd() {

    // TODO 3: Implement the logic for pwd
    
    if let  Ok(directory) = env::current_dir(){
    if let Some(directory_string) = directory.to_str() {
        println!("{}", directory_string);
        }
    }
}

fn echo(args : &Vec<String>) {

    let mut auxx = 0; // auxx is used to put space between the echo variables
    
    for i in args.iter() {

        if auxx == 0 {
            print!("{}", i);
            auxx = 1;
        }
        else {
            print!(" {}", i);
        }
    } 
}
fn cat(args : &Vec<String>) -> Result<i32, i32> {

    for i in args.iter() {

        match File::open(i) {

            Ok(mut file) => {
                let mut file_content:String = String::new();
               let _= file.read_to_string(&mut file_content);
                print!("{}",file_content);

                if !file_content.ends_with('\n') {
                    println!(); 
                }
            }
            Err(_error) => {
                return Err(-20);
                
            }
        }
    }

Ok(0)  
}
fn mkdir (args : &Vec<String>) -> Result<(), i32>{

    for i in args.iter() {
        match  fs::create_dir(i) {
            Ok(_v) => {

            }
            Err(_auxx) => {
                return Err(-30);
            }
            
        }
    }
Ok(())
}
fn mv (args : &Vec<String>) ->Result<(), i32>{

    match fs::rename(args[0].to_string(), args[1].to_string()) {
        Ok(_v) => {

        }
        Err(_err) =>{
            return Err(-40);
        }
    }

    Ok(())
}
fn ln(args : &Vec<String>) -> Result<i32, i32>{

    if args.len() < 2 {
        return Err(-50);
    }
    if args[0].to_string() == "-s"  || args[0].to_string() == "--symbolic"   {
        match symlink(args[1].to_string(), args[2].to_string()){
            Ok(_) => {
            }
            Err(_) =>{
                return Err(-50);
            }
        }
    }
    else {
        match fs::hard_link(args[0].to_string(), args[1].to_string()) {
            Ok(_) => {
            }
            Err(_err) =>{
                return Err(-50);
            }
        }
    }
Ok(0)
}
fn rmdir (args : &Vec<String>) -> Result<i32, i32> {

    for i in args.iter() {
        match fs::remove_dir(i.to_string()) {
            Ok(_) => {

            }
            Err(_err) => {
                return Err(-60);
            }

        }
        
    }
Ok(0)
}
fn rm (args : &Vec<String>) -> Result<i32, i32> {
    
    if args.len() < 2 {
        return Err(70);
    }
    let mut option1 = "-n";
    let mut option2 = "-n";
    let mut option3 = "-n";

    if args[0].to_string() == "-r"  || args[0].to_string() == "--recursive" || args[0].to_string() == "-R" {
        option2 = "-r";
        option1 = "c";
        }
    else if args[1].to_string() == "-r"  || args[1].to_string() == "--recursive" || args[1].to_string() == "-R" {
        option2 = "-r";
        option1 = "c";
        }
    if args[0].to_string() == "-d"  || args[0].to_string() == "--dir" {
        option3 = "-d";
        option1 = "c";
    }
    else if args[0].to_string() == "-d"  || args[0].to_string() == "--dir" {
        option3 = "-d";
        option1 = "c";
    }
        let mut  auxx = 1;
        for i in args.iter() {

            if i.starts_with('-') {
                continue; 
            }
            if let Ok(meta) = fs::metadata(i) {
                if meta.is_file() == true {
                    match fs::remove_file(i) {
                        Ok(_) => {

                        }
                        Err(_) => {
                            return Err(-70);
                        }
                    }

                }
                else if meta.is_dir() == true && option1 == "-n" {
                    auxx = 0;
                }
                else { 

                    if meta.is_dir() == true && option3 == "-r" {
                    match fs::remove_dir_all(i.to_string()) {
                        Ok(_) => {

                        }
                        Err(_) => {
                            return Err(-70);
                        }
                    }
                
                }
                if meta.is_dir() == true && option2 == "-d" && option3 != "r"{
                    if let Ok(mut path) = Path::new(i).read_dir() {
                        if path.next().is_none() {

                            match fs::remove_dir(i.to_string()) {
                                Ok(_) => {
                    
                                }
                                Err(_err) => {
                                    return Err(-70);
                                }
                    
                            }
                        }
                        else {
                            auxx = 0;
                        }
                    }
                    
                }
            }
            }
        }

        if auxx == 0 {
            return Err(-70);
        
    }

Ok(0) 

}
fn main() {

    // TODO 1: Read the command line arguments
    let args: Vec<String> = env::args().collect();

    // TODO 2: If the first argument is pwd, call pwd()
    if args[1] == "pwd" {
        pwd();
    }
    else {
        match args[1].as_str() {
            "echo" => {
                if  args.len() >= 3 && args[2] == "-n" {
                    echo(&args[3..].to_vec());
                    
                }
                else { if args.len() >= 2 {
                    echo(&args[2..].to_vec());
                    println!();
                }
            }

            }
            "cat" => { 
                if let Err(err) = cat(&args[2..].to_vec()){
                    std::process::exit(err);
                }
                else {
                    std::process::exit(0);
                }

                
            }
            "mkdir" => {
                if let Err(err) = mkdir(&args[2..].to_vec()) {
                    std::process::exit(err);
                }
            }
            "mv" => {
                if let Err(err) = mv(&args[2..].to_vec()) {
                    std::process::exit(err);
                }
            }
            "ln" =>{
                if let Err(err) = ln(&args[2..].to_vec()) {
                    std::process::exit(err);
                }
                else {
                    std::process::exit(0);
                }
            }
            "rmdir" => {
                if let Err(err) = rmdir(&args[2..].to_vec()) {
                    std::process::exit(err);
                }
                else {
                    std::process::exit(0);
                }
                
            }
            "rm" => {
                if let Err(err) = rm(&args[1..].to_vec()) {
                    std::process::exit(err);
                }
            }
            _ => {

            }

        }
    }
    
}