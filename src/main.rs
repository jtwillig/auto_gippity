use crate::helpers::command_line::get_user_response;

#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}

mod ai_functions;
mod apis;
mod helpers;
mod models;

fn main() {
    let usr_req = get_user_response("What webserver are we building today?");
    dbg!(usr_req);
}
