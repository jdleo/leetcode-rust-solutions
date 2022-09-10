use std::env;
use std::fmt::format;
use std::process::Command;

fn run(command: String) {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("error")
        .wait()
        .expect("error");
}

fn new(problem_id: String) {
    // format system commands
    let create = format!("touch solutions/src/problem_{}.rs", problem_id);
    let initstruct = format!(
        "echo 'struct Solution;' >> solutions/src/problem_{}.rs",
        problem_id
    );
    let lib = format!(
        "rm solutions/src/lib.rs && echo 'mod problem_{};' >> solutions/src/lib.rs",
        problem_id
    );
    let git = format!(
        "git add -A && git commit -m 'created problem {}'",
        problem_id
    );

    // run all commands
    run(create);
    run(initstruct);
    run(lib);
    run(git);
}

fn done(problem_id: String) {
    // format git command
    let git = format!(
        "git add -A && git commit -m 'finished problem {}'",
        problem_id
    );

    // run all commands
    run(git);
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
