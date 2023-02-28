use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{stdin, BufRead, BufReader, Write},
    path::PathBuf,
};

pub struct Query {
    main: String,
    sub: Option<String>,
}

pub fn show_cmd() {
    println!("welcome to Todo List!");
    println!("add [your todo] : input your Todos");
    println!("rm | remove [todo's id] : delete your Todos");
    println!("ls : show your Todos");
    println!("clear | cls : clear Todos.");
    println!("location | loc : show location");
}
pub fn new() -> PathBuf {
    let mut document_path: PathBuf =
        dirs::document_dir().expect("Err: Unable to access document path");
    document_path.push("TodoList");
    validate_dir(&document_path);

    //comming soon.. (logging capabilities)
    document_path.push("todo.txt");
    validate_file(&document_path);

    document_path
}

pub fn show_location(path: &PathBuf) {
    println!("location : \"{}\"\n", path.to_string_lossy())
}

pub fn set_query() -> Query {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(query) => {
            let sub_args: String = (&args[2..]).to_vec().join(" ");
            return Query {
                main: query.to_string(),
                sub: make_option(sub_args),
            };
        }
        None => {
            show_cmd();
            std::process::exit(0)
        }
    }
}

pub fn get_main_query(query: &Query) -> String {
    query.main.clone()
}

pub fn list_add(path: &PathBuf, query: Query) {
    let sub = match query.sub {
        Some(sub) => sub,
        None => {
            println!("Insufficient content!");
            show_cmd();
            return;
        }
    };
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    write!(file, "{}\n", sub).expect("Err: Unable to write file.");
    println!("( {} ) was added!", sub);
}

pub fn list_remove(path: &PathBuf, query: Query) {
    let sub = match query.sub {
        Some(sub) => sub.parse::<u32>().unwrap(),
        None => {
            println!("Insufficient content!");
            show_cmd();
            return;
        }
    };
    let mut todos: String = String::new();
    let mut count: u32 = 1;
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        if sub != count {
            todos.push_str(&format!("{}\n", line.unwrap()))
        } else {
            println!("( {} ) has been removed!", line.unwrap());
        }
        count += 1;
    }
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Err: Unable to open file")
        .write_all(todos.as_bytes())
        .expect("Err: Unable to open file");
}

pub fn list_segments(path: &PathBuf) {
    let file: File = File::open(path).unwrap();
    let mut count: u32 = 1;
    for line in BufReader::new(file).lines() {
        println!("todo {}: {}", count, line.unwrap());
        count += 1;
    }
    println!("total todos: {}", count - 1);
}

pub fn list_clear(path: &PathBuf) {
    println!("Are you sure you want to remove all? (y/n)");
    let mut reconfirm: String = String::new();
    stdin()
        .read_line(&mut reconfirm)
        .ok()
        .expect("Err: Failed to read line");
    reconfirm = reconfirm.chars().next().unwrap().to_string();

    if reconfirm.eq_ignore_ascii_case("y") {
        fs::remove_file(path.clone()).expect("Err: Failed to delete file");
        println!("List cleared");
    } else {
        println!("List lear cancelled");
    }
}
fn validate_dir(path: &PathBuf) {
    if !path.exists() {
        println!("created dir at \"{}\"!", path.display());
        std::fs::create_dir(path).unwrap();
    }
}
fn validate_file(path: &PathBuf) {
    if !path.exists() {
        println!("created txt file at \"{}\"!", path.display());
        File::create(path).unwrap();
    }
}
fn make_option(input: String) -> Option<String> {
    if input.eq("") {
        return None;
    } else {
        return Some(input);
    }
}
