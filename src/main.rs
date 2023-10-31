mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

fn main() {
    println!("Hello, user,");

    let user_request: String = get_user_response("What webserver are we building today?");
    dbg!(user_request);
}
