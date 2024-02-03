use std::fs;
use std::path::Path;

fn init(repo_name: &str) -> Result<(), std::io::Error> {
    // create dir for repo and initialize .git/
    if let Err(e) = fs::create_dir(repo_name) {
        return Err(e);
    }

    // create .git/
    let git_dir = repo_name.to_owned() + "/.git/";
    if let Err(e) = fs::create_dir(git_dir) {
        return Err(e);
    }

    Ok(())
}

fn main() {
    match init("test_repo") {
        Ok(_) => println!("Repository initialized successfully"),
        Err(e) => eprintln!("Error initializing repository: {}", e),
    }
}
