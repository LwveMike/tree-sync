use std::ffi::OsStr;
use std::fs::ReadDir;
use std::{fs};
use std::path::{Path, PathBuf};
use std::process;

static ROOT_DIR: &str = "/Users/admin/Projects/tree-sync/src";

fn main () {
    start();
}

// TODO: add section that validates number of args with number of values to be replaced
fn add_values_to_template (template: String, values: &Vec<String>) -> String {
    let mut transformed_template = template.clone();
    for value in values {
        transformed_template = transformed_template.replacen("{{}}", value, 1);
    }

    transformed_template
}

fn get_template_dir (template_dir_path: &Path) -> ReadDir {
    if !template_dir_path.exists() {
        eprintln!("No entity found at path {}", template_dir_path.display());
        process::exit(1)
    }

    if !template_dir_path.is_dir() {
        eprintln!("Entity is not of type dir");
        process::exit(1);
    }

    match fs::read_dir(template_dir_path) {
        Ok(dir) => { dir },
        Err(_) => {
            eprintln!("Couldn't read directory");
            process::exit(1);
        }
    }
}

fn get_templates (template_dir: ReadDir) -> Vec<PathBuf> {
    let mut templates_paths: Vec<PathBuf> = Vec::new();

    for dir_entry in template_dir {
        let path = dir_entry.expect("Dir entry error").path();

        if !path.is_file()  {
            eprintln!("You can't have dirs in template dir");
            process::exit(1);
        }

        match path.extension().and_then(OsStr::to_str) {
            Some(extension) => {
                if extension != "template" {
                    eprintln!("You can't have files that have a extension different than .template");
                    process::exit(1);
                }
            }
            None => {
                eprintln!("You can't have files that doesn't have an extension");
                process::exit(1);
            }
        }

        templates_paths.push(path);
    }

    templates_paths
}

// fn print_templates_content (template_dir_path: &String) -> () {
//     let templates_paths = get_templates(
//         get_template_dir(Path::new(template_dir_path))
//     );

//     for template_path in templates_paths {
//         let template = read_template(template_path);
//         let values = vec!["useEmitter".to_string()];
//         println!("{}", add_values_to_template(template, &values))
//     }
// }

// fn print_args () {
//     let mut args: Vec<_> = std::env::args().collect();
//     args.remove(0);

//     for (index, arg) in args.iter().enumerate() {
//         println!("{} \n", index);
//     }
// }

fn read_template (path: &String) -> String {
    let template_path = Path::new(ROOT_DIR).join(path);

    match fs::read(template_path) {
        Ok(file_octets) => {
            match String::from_utf8(file_octets){
                Ok(file_content) => {
                    if file_content.is_empty() {
                        eprintln!("You can't have empty template");
                        process::exit(1);
                    }

                    file_content
                },
                Err(_) => {
                    eprintln!("Can't convert octets to to utf-8 string");
                    process::exit(1);
                }
            }
        }
        Err(_) => {
            eprintln!("Can't read file's octets");
            process::exit(1);
        }
    }
}

fn start () -> () {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 | 2 | 3  => {
            eprintln!("Should provide more arguments");
            process::exit(1);
        }
        4 => {
            let command = &args[1];

            if command == &String::from("new") {
                let template_path = args[2].clone() + ".template";
                let file_content = read_template(&template_path);
                let values = vec!(args[3].clone());

                let for_writing = add_values_to_template(file_content, &values);


                println!("Everything is good");
                process::exit(0);
            }

            todo!("Implement command");
        }

        _ => {
            eprintln!("Too many arguments provided");
            process::exit(1);
        }
    }
}


