use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo <command> [options]");
        return;
    }

    match args[1].as_str() {
        "init" => init_command(&args),
        "add" => add_command(&args),
        "show" => show_command(),
        "remove" => remove_command(&args),
        _ => println!("Unknown command"),
    }
}

fn init_command(args: &[String]) {
    if args.len() < 4 || args[2] != "-p" {
        println!("Usage: todo init -p <path>");
        return;
    } 
    // check arg index 2 is not -p and arge index 3 is empty or doesn't exist
    else if args[2] != "-p" && args[3].is_empty() {
        
    }
    let path = &args[3];
    println!("TODO list initialized on path: {}", path);

    // write empty file to new variable path
    let mut file: File = File::create(path).expect("Unable to create file");
    
    let title: &String = &args[4];
    let content = format!("# {}\n", title);
    
    file.write_all(content.as_bytes()).expect("Unable to write to file");
}

fn add_command(args: &[String]) {
    let mut message = String::new();
    let mut description = String::new();
    let mut msg_flag = false;
    let mut desc_flag = false;

    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "-m" => msg_flag = true,
            "-d" => desc_flag = true,
            _ if msg_flag => {
                message = arg.to_string();
                msg_flag = false;
            },
            _ if desc_flag => {
                description = arg.to_string();
                desc_flag = false;
            },
            _ => {}
        }
    }

    if message.is_empty() || description.is_empty() {
        println!("Usage: todo add -m 'task_name' -d 'description'");
        return;
    }

    println!("Adding task: {}, Description: {}", message, description);
}

fn show_command() {
    println!("Displaying all tasks...");
}

fn remove_command(args: &[String]) {
    if args.len() != 3 {
        println!("Usage: todo remove -n <number>");
        return;
    }
    let number = &args[2];
    println!("Removing task number: {}", number);
}

fn create_file(relative_path: &str) -> io::Result<()> {
    let mut path: PathBuf = std::env::current_dir()?;

    path.push(relative_path); // extends current dir to add the relative path

    
}