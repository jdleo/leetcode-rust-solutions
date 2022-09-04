use std::env;
use std::process::Command;

fn new(problem_id: String) {
    // format mkdir, touch, git, and open commands
    let mkdir = format!("mkdir solutions/{}", problem_id);
    let touch = format!("touch solutions/{}/main.rs", problem_id);
    let git = format!(
        "git add solutions/{}/main.rs && git commit -m \"create {}\"",
        problem_id, problem_id
    );

    // make folder under /solutions
    Command::new("sh")
        .arg("-c")
        .arg(mkdir)
        .spawn()
        .expect("error")
        .wait()
        .expect("error");

    // create main.rs
    Command::new("sh")
        .arg("-c")
        .arg(touch)
        .spawn()
        .expect("error")
        .wait()
        .expect("error");

    // commit
    Command::new("sh")
        .arg("-c")
        .arg(git)
        .spawn()
        .expect("error")
        .wait()
        .expect("error");
}

fn done(problem_id: String) {
    // format git command
    let git = format!(
        "git add solutions/{}/main.rs && git commit -m \"done with {}\"",
        problem_id, problem_id
    );

    // git command
    Command::new("sh")
        .arg("-c")
        .arg(git)
        .spawn()
        .expect("error")
        .wait()
        .expect("error");
}

fn main() {
    // get args
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];
    let problem_id = &args[2];

    // figure out which command we want
    match &cmd[..] {
        "new" => new(String::from(problem_id)),
        "done" => done(String::from(problem_id)),
        _ => {
            eprintln!("Invalid command...")
        }
    }
}
