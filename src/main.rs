use std::fs;
use std::path::Path;

fn init(repo_name: &str) -> Result<(), std::io::Error> {
    // create dir for repo and initialize .git/
    if let Err(e) = fs::create_dir(repo_name) {
        return Err(e);
    }

    // create .git/ folders
    let primary_git_dir = repo_name.to_owned() + "/.git/";
    if let Err(e) = fs::create_dir(&primary_git_dir) {
        return Err(e);
    }

    let git_dirs: [&str; 3] = ["/objects/", "/refs/", "/refs/heads/"];
    for dir in git_dirs {
        match fs::create_dir(primary_git_dir.clone() + dir) {
            Ok(_) => println!("Directory '{}' created successfully", dir),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn main() {
    match init("test_repo") {
        Ok(_) => println!("Repository initialized successfully"),
        Err(e) => eprintln!("Error initializing repository: {}", e),
    }
}
