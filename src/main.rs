use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;
use std::path::Path;

fn init(repo_name: &str) -> Result<(), std::io::Error> {
    // create dir for repo and initialize .git/
    fs::create_dir(repo_name)?;

    // create .git/ folders
    let primary_git_dir = repo_name.to_owned() + "/.git/";
    fs::create_dir(&primary_git_dir)?;

    let git_dirs: [&str; 3] = ["/objects/", "/refs/", "/refs/heads/"];
    for dir in git_dirs {
        match fs::create_dir(primary_git_dir.clone() + dir) {
            Ok(_) => println!("Directory '{}' created successfully", dir),
            Err(e) => return Err(e),
        }
    }
    let mut head = fs::File::create(primary_git_dir + "/HEAD")?;
    head.write_all(b"ref: refs/heads/main")?;

    Ok(())
}

fn hash_object(data: Vec<u8>, obj_type: &str, write: bool) -> Result<String, std::io::Error> {
    // this entire function is basically an error
    let header = format!("{} {}", obj_type, data.len()).into_bytes();
    let all_data = [header, b"\x00".to_vec(), data].concat(); // FIXME: What.!

    let mut hasher = Sha1::new();
    hasher.update(&all_data);
    let result = hasher.finalize();

    if write {
        let path_str = String::from_utf8([b".git/objects/", &result[..2], &result[2..]].concat());
        let path_str = path_str.unwrap();
        let path = Path::new(&path_str);

        if !path.exists() {
            fs::create_dir_all(path)?;
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write_all(&all_data)?;
            let compressed_data = e.finish().unwrap();

            let file = fs::File::create(path);
            file.unwrap().write_all(&compressed_data)?
        }
    }

    Ok(format!("{:x}", result))
}

fn main() {
    match init("test_repo") {
        Ok(_) => println!("Repository initialized successfully"),
        Err(e) => eprintln!("Error initializing repository: {}", e),
    }
}
