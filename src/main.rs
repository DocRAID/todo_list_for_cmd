use std::{path::PathBuf, fmt::Error};

fn main() {
    let path = Setting::new();
    
}
struct Setting {
    query:String,
    sub_query:Option<String>
}
impl Setting {
    fn new () -> Result<PathBuf,Error> {
        let mut document_path = dirs::document_dir().unwrap();
        document_path.push("todoList");
        document_path.push("todo.txt");
        Ok(document_path)
    }
    fn show_cmd(){
        println!("welcome to Todo List!");
        println!("add [your todo] : input your Todos");
        println!("rm [todo's id] : delete your Todos");
        println!("ls : show your Todos");
        println!("clear : clear Todos.");
    }
}