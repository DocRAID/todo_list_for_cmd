use command::Query;
mod command;
fn main () {
    let query: Query = command::get_args();
}