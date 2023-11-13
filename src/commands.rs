// The project (without optionnal) require few commands :
//
// echo
// cd
// ls
// pwd
// cat
// cp
// rm
// mv
// mkdir
// exit

use std::fs::{self, OpenOptions};

pub enum Commands {
    Echo,
    Cd,
    Ls,
    Pwd,
    Cat,
    Cp,
    Rm,
    Mv,
    Mkdir,
    Exit,
    Touch,
    Clear,
}

impl Commands {
    pub fn parse(input: &str) -> Result<Self, String> {
        // support for arguments
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.get(0) {
            Some(&"echo") => Ok(Commands::Echo),
            Some(&"exit") => Ok(Commands::Exit),
            Some(&"pwd") => Ok(Commands::Pwd),
            Some(&"cat") => Ok(Commands::Cat),
            Some(&"cp") => Ok(Commands::Cp),
            Some(&"touch") => Ok(Commands::Touch),
            Some(&"mkdir") => Ok(Commands::Mkdir),
            Some(&"ls") => Ok(Commands::Ls),
            Some(&"rm") => Ok(Commands::Rm),
            Some(&"cd") => Ok(Commands::Cd),
            Some(&"mv") => Ok(Commands::Mv),
            Some(&"clear") => Ok(Commands::Clear),
            _ => Err("Command not supported yet".to_string()),
        }
    }

    pub fn execute(&self, args: &[String]) {
        match self {
            Commands::Echo => execute_echo(args),
            Commands::Exit => execute_exit(),
            Commands::Pwd => execute_pwd(),
            Commands::Cat => execute_cat(args),
            Commands::Cp => execute_cp(args),
            Commands::Touch => execute_touch(args),
            Commands::Mkdir => execute_mkdir(args),
            Commands::Ls => execute_ls(args),
            Commands::Rm => execute_rm(args),
            Commands::Cd => execute_cd(args),
            Commands::Mv => execute_mv(args),
            Commands::Clear => execute_clear(),
        }
    }
}

/// Execute the `echo` command.
///
/// # Examples
///
/// ```
/// $ echo "Hi Rustcean!"
/// ```
/// ## Output
///
/// ```
/// $ Hi Rustcean!
/// ```
pub fn execute_echo(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage : echo [something_to_echo]");
        return;
    }

    let clean_args: Vec<String> = args
        .iter()
        .map(|arg| arg.replace('\"', "").replace('\'', ""))
        .collect();

    let output = clean_args.join(" ");
    println!("{}", output);
}

/// Execute the `exit` command.
///
/// This command simply exit the shell.
/// (Note : 0 is a success code. Everything else is an error)
pub fn execute_exit() {
    std::process::exit(0)
}

/// Execute the `pwd` command. (Print Working Directory)
///
/// This function is not based on `env::current_dir` from the standard Rust library. Instead, it
/// directly reads the symbolic link "/proc/self/cwd" to obtain the path of the current working
/// directory.
///
/// # Examples
///
/// ```
/// $ pwd
/// ```
/// ## Output
///
/// ```
/// $ /home/Rustcean/Work/rust-basic/
/// ```
pub fn execute_pwd() {
    match fs::read_link("/proc/self/cwd") {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                println!("{}", path_str);
            } else {
                eprintln!("Unable to convert path to string.");
            }
        }
        Err(e) => {
            eprintln!("Error reading current directory : {}", e);
        }
    }
}

/// Execute the `cat` command.
///
/// This function processes multiple file names provided as arguments. It uses a loop to iterate
/// through each argument. Each argument is expected to be a file name, and the function attempts
/// to read and print the contents of each file sequentially.
///
/// # Examples
///
/// ```
/// $ cat secret.txt
/// ```
/// ## Output
/// ```
/// $ This is my secret
/// ```
pub fn execute_cat(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: cat [file_to_cat]");
        return;
    }

    for arg in args {
        match fs::read_to_string(arg) {
            Ok(content) => {
                println!("{}", content);
            }
            Err(e) => {
                eprintln!("Error reading file '{}' : {}", arg, e);
            }
        }
    }
}

/// Execute the `cp` command.
///
/// Todo: When using `cp` command, let the user choose only a directory to copy and remove the fact
/// that he need to input a new name for his file to copy.
///
/// # Examples
///
/// ```
/// $ cp file_to_copy folder/name_of_the_file
/// ```
pub fn execute_cp(args: &[String]) {
    if args.len() != 2 {
        eprintln!("Usage: cp [file_src] [file_dest]");
        return;
    }

    let file_to_copy = args.get(0).unwrap();
    let path = args.get(1).unwrap();

    match std::fs::copy(file_to_copy, path) {
        Ok(_) => {
            println!("File copied successfully");
        }
        Err(e) => {
            eprintln!("Failed to copy file : {}", e);
        }
    }
}

/// Execute the `touch` command.
///
/// # Examples
/// ```
/// $ touch hello
/// ```
/// ## Output
/// ```
/// $ ls
/// $ hello 42.txt
/// ```
pub fn execute_touch(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: touch [file_name]");
        return;
    }

    let file = args.get(0).unwrap();

    match OpenOptions::new().create(true).write(true).open(file) {
        Ok(_) => {
            println!("File created successfully");
        }
        Err(e) => {
            eprintln!("Failed to create file : {}", e);
        }
    }
}
/// Execute the `mkdir` command.
///
/// # Examples
/// ```
/// $ mkdir my_folder
/// ```
/// ## Output
/// ```
/// $ ls
/// $ my_folder/ 42.txt
/// ```
pub fn execute_mkdir(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage : mkdir [folder_name]");
        return;
    }

    for arg in args {
        match fs::create_dir(arg) {
            Ok(_) => {
                println!("Folder created successfully");
            }
            Err(e) => {
                eprintln!("Failed to create the new folder : {}", e);
            }
        }
    }
}
/// Execute the `ls` command.
///
/// # Examples
/// ```
/// $ ls
/// ```
/// ## Output
/// ```
/// $ src/
/// $ .git/
/// $ README.md
/// ```
pub fn execute_ls(args: &[String]) {
    if args.is_empty() {
        list_directory(".")
    } else {
        for arg in args {
            list_directory(arg);
        }
    }
}

fn list_directory(path: &str) {
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }
        Err(e) => {
            eprintln!(
                "Failed to read the content of the directory '{}': {}",
                path, e
            );
        }
    }
}

/// Execute the `cd` command.
///
///
/// # Examples
/// ```
/// $ pwd
/// $ /home/user/my_project
/// $ cd ..
/// ```
///
/// ## Output
/// ```
/// $ pwd
/// $ /home/user
/// ```
pub fn execute_cd(args: &[String]) {
    if args.len() != 1 {
        eprintln!("Usage : cd [directory_to_move]");
        return;
    }

    for arg in args {
        match std::env::set_current_dir(arg) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to change directory : {}", e);
            }
        }
    }
}

/// Execute the `mv` command.
///
///
/// # Examples
/// ```
/// $ touch helo 
/// $ mv helo hello 
/// ```
///
/// ## Output
/// ```
/// $ ls
/// $ hello 
/// ```
pub fn execute_mv(args: &[String]) {
    if args.len() != 2 {
        eprintln!("Usage: mv [file_src] [file_dest]");
        return;
    }

    let file_to_rename = args.get(0).unwrap();
    let new_filename = args.get(1).unwrap();

    match std::fs::rename(file_to_rename, new_filename) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to rename the file : {}", e);
        }
    }
}

/// Execute the `rm` command.
///
/// Todo:
/// Add flag to remove folder too.
///
/// # Examples
/// ```
/// $ rm file_to_remove
/// ```
pub fn execute_rm(args: &[String]) {
    for arg in args {
        match fs::remove_file(arg) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Eror when removing the file : {}", e);
            }
        }
    }
}

/// Execute the `clear` command.
///
/// Simply clear the screen.
///
pub fn execute_clear() {
    clearscreen::clear().unwrap()
}
