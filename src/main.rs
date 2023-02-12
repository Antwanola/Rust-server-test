#[macro_use] extern crate nickel;

use nickel:: Nickel;

fn say_hello()-> &'static str {
    "Hellow Bobby!!!"
}

fn main() {
    let mut server =  Nickel::new();
    server.utilize(router!{
        get "**" => |_req, _res | {
           say_hello()
        }
    });
    server.listen("127.0.0.1:8080").unwrap();
}
