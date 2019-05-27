use std::env; // for collecting command line arguments
use std::str; // for string handling
use std::process::Command; // for executing bash commands

fn main() {
    // get all of the command line arguments
    let mut args: Vec<String> = env::args().collect();
    // replace this program's binary execution command with "docker" 
    args[0] = "docker".to_string();
    
    // make system call based on arguments
    let docker_cmd = Command::new(&args[0])
            .arg(&args[1])
            .output()
            .expect("The process failed to execute");

    // collect stdout and convert to string
    let docker_out = docker_cmd.stdout;
    let out_str = match str::from_utf8(&docker_out){
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{:?}", args);
    println!("{}", out_str);
}
