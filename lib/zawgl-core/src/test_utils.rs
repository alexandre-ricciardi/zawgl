// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::path::Path;

const WIN_TEST_DIR: &str = "C:\\Temp";
const UNIX_TEST_DIR: &str = "/tmp";

const UNIX_SEPARATOR: &str = "/";
const WIN_SEPARATOR: &str = "\\";

pub fn get_tmp_dir_path(dir_name: &str) -> String {
    let dir;
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        dir = format!("{}/{}", UNIX_TEST_DIR, dir_name);
    } else {
        dir = format!("{}\\{}", WIN_TEST_DIR, dir_name);
    }
    dir
}


pub fn build_dir_path_and_rm_old(dir_name: &str) -> std::io::Result<String> {
    let dir = get_tmp_dir_path(dir_name);
    let path = Path::new(&dir);
    if path.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn build_file_path_and_rm_old(dir_name: &str, file_name: &str) -> std::io::Result<String> {
    let dir;
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        dir = format!("{}/{}", UNIX_TEST_DIR, dir_name);
    } else {
        dir = format!("{}\\{}", WIN_TEST_DIR, dir_name);
    }
    std::fs::create_dir_all(&dir)?;
    let file;
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
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