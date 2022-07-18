use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

fn main() -> Result<(),Error>{
    //쿼리를 받아오고
    let mut args: Vec<String> = env::args().collect();
    //쿼리 유효성 검사
    check_up(1, args.clone());
    let query = &args[1];

    match query.as_str(){
        "add" =>{
            check_up(2, args.clone());
            let mut to_add = String::new();
            args.remove(0);
            args.remove(1);
            for i in args {
                to_add.push_str(&i);
                to_add.push_str(" ");
            }
            add_list(to_add);
        },
        "del" =>{
            check_up(2, args.clone());
            let to_del = &args[2];
            del_list(to_del);
        },
        "ls" => {
            view_list();
        },
        "clear" => {
            clear_list();
        },
        _=>show_cmd() //wrong query
    }
    Ok(())
}

fn show_cmd(){
    println!("welcome to Todo List!");
    println!("add [your todo] : input your Todos");
    println!("del [todo Id] : delete your Todos");
    println!("ls : show your Todos");
    println!("clear : clear Todos");
}
fn check_up(index:usize, mut arr:Vec<String>){ //쿼리가 잘 되어있는지 확인.
    match arr.get_mut(index){
        Some(_x) => print!(""),
        None => {
            show_cmd();
            std::process::exit(0)
        }
    }
}

fn add_list(to_add:String) -> Result<(),Error> {
    println!("( {} ) was added!",to_add);
    let path = "todo.txt";
    let mut output = File::create(path)?;
    write!(output, "{}",to_add)?;
    
    // todo!("read index and write on file");
    Ok(())
}
fn del_list(to_del:&str){
    todo!("read index and write on file");
}
fn view_list(){
    let file = File::open("todo.txt").unwrap();
    let mut count:u32=1;
    for line in BufReader::new(file).lines() {
        println!("todo {}: {}", count, line.unwrap());
        count+=1;
    }
    println!("total {}",count);
    
}
fn clear_list() {
    todo!("clear list")
}
