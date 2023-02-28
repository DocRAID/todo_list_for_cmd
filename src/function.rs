use std::{env, fmt::Error, fs::File, path::PathBuf};

pub struct Query {
    main: String,
    sub: Option<String>,
}

pub fn new() -> Result<PathBuf, ()> {
    let mut document_path = dirs::document_dir().expect("Err: Unable to access document path");
    document_path.push("TodoList");
    validate_dir(&document_path);
    document_path.push("todo.txt");
    validate_file(&document_path);
    //file exist check
    // Ok(document_path)
    Ok(document_path)
}
pub fn show_cmd() {
    println!("welcome to Todo List!");
    println!("add [your todo] : input your Todos");
    println!("rm [todo's id] : delete your Todos");
    println!("ls : show your Todos");
    println!("clear : clear Todos.");
}
pub fn show_location(path: &PathBuf) {
    println!("location : \"{}\"\n",path.to_string_lossy())
}
pub fn set_query() -> Query {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(query) => {
            return Query {
                main: query.to_string(),
                sub: args.get(2).cloned(),
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
pub fn list_add(path: &PathBuf,query:Query) {}
pub fn list_remove(path: &PathBuf,query:Query) {}
pub fn list_segments(path: &PathBuf) {}
pub fn list_clear(path: &PathBuf) {}

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
