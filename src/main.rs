use function::Query;
use std::path::PathBuf;

mod function;
fn main() {
    let path: PathBuf = function::new().expect("Err: Unable to access document directory.");
    let query: Query = function::set_query();

    match function::get_main_query(&query).as_str() {
        "add" => {
            function::list_add(&path, query);
        }
        "rm" | "remove" => {
            function::list_remove(&path, query);
        }
        "ls" => {
            function::list_segments(&path);
        }
        "clear" | "cls" => {
            function::list_clear(&path);
        }
        "location" | "loc" => {
            function::show_location(&path);
        }
        _ => {
            println!("Command does not exist.\nFollow the instructions below.");
            function::show_cmd();
        }
    }
    return;
}
