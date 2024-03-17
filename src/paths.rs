use std::env;
use std::path::Path;
use crate::log;

pub fn get_current_path() -> String {
    let get = env::current_dir();
    match get {
        Ok(t) => t.to_str().unwrap().to_string(),
        Err(_) => todo!(),
    }
}

pub fn get_parent_path() -> String {
    let current_folder = get_current_path();
    let path = Path::new(&current_folder);
    path.parent().unwrap().display().to_string()
}

pub fn is_path_correct(debug : bool) -> bool {
    let mut result : bool = false;
    let dll: String = get_parent_path()+ "\\onefile.dll";

    let log_text: &str = dll.as_str();
    log::add(log_text.to_owned(), debug);

    let dll_path: &Path = Path::new(&dll);
    if Path::exists(dll_path){
        result = true;
    }
    result
}