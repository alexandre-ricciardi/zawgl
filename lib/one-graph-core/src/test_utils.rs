use std::path::Path;

const WIN_TEST_DIR: &str = "C:\\Temp";
const UNIX_TEST_DIR: &str = "/tmp";

const UNIX_SEPARATOR: &str = "/";
const WIN_SEPARATOR: &str = "\\";

pub fn build_file_path_and_rm_old(dir_name: &str, file_name: &str) -> std::io::Result<String> {
    let dir;
    if cfg!(target_os = "linux") {
        dir = format!("{}/{}", UNIX_TEST_DIR, dir_name);
    } else {
        dir = format!("{}\\{}", WIN_TEST_DIR, dir_name);
    }
    std::fs::create_dir_all(&dir)?;
    let file;
    if cfg!(target_os = "linux") {
        file = format!("{}/{}", &dir, file_name);
    } else {
        file = format!("{}\\{}", &dir, file_name);
    }
    let path = Path::new(&file);
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    Ok(file)
}