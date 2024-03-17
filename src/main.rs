use crate::webhooks::send_webhook;
use chrono::prelude::*;
use paths::is_path_correct;
use std::env;
use std::process;

extern crate exitcode;

use crate::vars::DEVELOPER;
use crate::vars::D_EXPIRE;
use crate::vars::TITLE;
use crate::vars::VERSION;

mod paths;
mod vars;
mod log;
mod webhooks;

#[tokio::main]
async fn main() {
    // Useful ini vars
    let mut debug: bool = false;
    let now: NaiveDate = Utc::now().date_naive();
    let naive_date = NaiveDate::parse_from_str(D_EXPIRE, "%Y-%m-%d").unwrap();

    // ARGUMENTS
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        process::exit(exitcode::CONFIG);
    }

    // debug variable
    for i in 1..args.len() {
        if args[i] == "-d" || args[i] == "--debug" {
            debug = true;
        }
    }

    // All necessary args
    let mut _tokenid = String::new();
    let mut _token = String::new();
    let mut _account = String::new();
    let mut _msg = String::new();
    let mut _filepath = String::new();

    for i in 1..args.len() {
        //log::setlog(args[i].to_string(), debug);
        if args[i] == "-v" || args[i] == "--version" {
            println!("Current version is v{} by {}", VERSION, DEVELOPER);
            process::exit(exitcode::OK);
        }
        if args[i] == "-h" || args[i] == "--help" {
            println!("arguments to use are >> id= token= account= msg= title= filepath= ");
            process::exit(exitcode::OK);
        }
        if args[i].to_uppercase().contains("ID=") {
            _tokenid = args[i].to_uppercase().replace("ID=", "").to_string();
            log::add("id : ".to_string() + &_tokenid, debug);
        }
        if args[i].to_uppercase().contains("TOKEN=") {
            _token = args[i].to_uppercase().replace("TOKEN=", "").to_string();
            log::add("token : ".to_string() + &_token, debug);
        }
        if args[i].to_uppercase().contains("ACCOUNT=") {
            _account = args[i].to_uppercase().replace("ACCOUNT=", "").to_string();
            log::add("account : ".to_string() + &_account, debug);
        }
        if args[i].to_uppercase().contains("MSG=") {
            _msg = args[i].to_uppercase().replace("MSG=", "").to_string();
            log::add("msg : ".to_string() + &_msg, debug);
        }
        if args[i].to_uppercase().contains("FILEPATH=") {
            _filepath = args[i].to_uppercase().replace("FILEPATH=", "").to_string();
            log::add("filepath : ".to_string() + &_filepath, debug);
        }
    }

    if !is_path_correct(debug){
        println!("lib path please contact developer!");
        process::exit(exitcode::DATAERR);
    }

    // was expired
    if now > naive_date {
        println!("lib has expired please contact developer!");
        process::exit(exitcode::DATAERR);
    }

    if now <= naive_date {
        // Title
        println!("{TITLE}");

        log::add(
            "version is ".to_string() + VERSION + " by " + DEVELOPER,
            debug,
        );
        log::add(
            "current path is ".to_string() + &paths::get_current_path(),
            debug,
        );
        log::add(
            "current parent path is ".to_string() + &paths::get_parent_path(),
            debug,
        );
    }

    send_webhook(&_tokenid, &_token, &_filepath, &_account, &_msg).await;
}
