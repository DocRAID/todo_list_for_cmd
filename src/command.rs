//query fillter, select, etc...

use std::env;

pub struct Query {
    main: String,
    sub: Option<String>,
}

pub fn get_args() -> Query {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(query) => {
            let sub_args: String = (&args[2..]).to_vec().join(" ");
            return Query {
                main: query.to_string(),
                sub: if sub_args.eq("") {
                    None
                } else {
                    Some(sub_args)
                },
            };
        }
        None => {
            show_cmd();
            std::process::exit(0)
        }
    }
}
pub fn show_cmd() {
    println!("welcome to Todo List!");
    println!("add [your todo] : add your Todo");
    println!("remove | rm [todo id] : remove your Todo");
    println!("ls : show your Todo");
    println!("clear | cls : clear all Todo");
    println!("undo : backup of removed Todo (just once)");
    println!("config | conf : show path of config file");
}