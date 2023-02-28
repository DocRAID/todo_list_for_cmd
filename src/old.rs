extern crate dirs;
use std::path::PathBuf;
use std::{env, fs, path};
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead, Error, stdin};

fn main() -> Result<(),Error>{
    let mut document_path = dirs::document_dir().expect("can't access path");
    document_path.push("todo.txt");//path

    //파일 유효성 검사
    file_exist(document_path.clone()).expect("err on file exist checking");
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
            args.remove(0);
            for i in args {
                to_add.push_str(&i);
                to_add.push_str(" ");
            }
            add_list(to_add,document_path).expect("Error on (fn)add_list");
        },
        "rm" =>{
            check_up(2, args.clone());
            let to_remove = &args[2];
            remove_list(to_remove,document_path).expect("Error on (fn)remove_list");
        },
        "ls" => {
            view_list(document_path);
        },
        "clear" => {
            clear_list(document_path).expect("Error on (fn)clear_list");
        },
        _=>show_cmd() //wrong query
    }

    Ok(())
}

fn show_cmd(){
    println!("welcome to Todo List!");
    println!("add [your todo] : input your Todos");
    println!("rm [todo's id] : delete your Todos");
    println!("ls : show your Todos");
    println!("clear : clear Todos.");
}

fn file_exist(document_path:PathBuf) -> Result<(),Error>{ //파일이 존재하는지 확인.
    // let path = "todo.txt";
    let exits = document_path.exists();

    if !exits {
        println!("created file!");
        File::create(document_path)?;
    }
    Ok(())
}
fn check_up(index:usize, mut arr:Vec<String>){ //쿼리가 잘 되어있는지 확인.
    match arr.get_mut(index){
        Some(_x) => print!(""),
        None => {
            show_cmd();
            std::process::exit(0) //쿼리가 불완전하면 명령어를 보여주고 종료.
        }
    }
}
fn add_list(to_add:String,document_path:PathBuf) -> Result<(),Error> { //add
    println!("( {}) was added!",to_add);
    let mut file = fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open(document_path)
    .unwrap();

    write!(file, "{}\n",to_add)
    .expect("[Error_log] : can't write file...");
    Ok(())
}

fn remove_list(to_remove_str:&str,document_path:PathBuf) -> Result<(),Error>{  //del
    let to_remove = to_remove_str.parse::<i32>().unwrap();
    // todo!("to_remove = index read index and write on file");
    let file = File::open(document_path.clone())?;
    let reader = BufReader::new(file);
    let mut todos = String::new();
    let mut count = 1;
    for line in reader.lines() {
        if to_remove != count {
            todos.push_str(&format!("{}",line?));
            todos.push_str("\n")
        }else{
            println!("( {} ) has been removed!",line?);
        }
        count+=1;
    }
    let mut write = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(document_path)
        .expect("Unable to open file");
        
    write
        .write_all(todos.as_bytes())
        .expect("Unable to write data");
    Ok(())
}

fn view_list(document_path:PathBuf){
    let file = File::open(document_path).unwrap();
    let mut count:u32=1;
    for line in BufReader::new(file).lines() {
        println!("todo {}: {}", count, line.unwrap());
        count+=1;
    }
    println!("total todos : {}",count-1);  
}

fn clear_list(document_path:PathBuf) -> Result<(),Error>{
    println!("you want clear file? (y/n)");

    let mut input_string = String::new();

    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");
    input_string = input_string.chars().next().unwrap().to_string();
    if input_string.eq("Y") || input_string.eq("y"){
        std::fs::remove_file(document_path.clone()).expect("Error on removing file");
        println!("Finished cleaning todo!");
        File::create(document_path)?;
    }
    else {
        println!("canceled cleaning todo!");
    }
    Ok(())
}
