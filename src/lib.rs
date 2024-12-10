// take a directory

// recursively copy files and directories in target directory
// .css
// .js

use std::path;
use std::path::PathBuf;
use tokio::fs::{read_dir, ReadDir};

// depth first
// can have max depth

/*
    one stack, stack of iterators
*/

pub struct DirWalk {
    path_stack: Vec<ReadDir>,
}

impl DirWalk {
    pub async fn try_from_path(source_path: &PathBuf) -> Result<DirWalk, String> {
        let path_buf = match path::absolute(source_path) {
            Ok(p) => p,
            Err(e) => return Err(e.to_string()),
        };

        let dir_entries = match read_dir(&path_buf).await {
            Ok(rd) => rd,
            Err(e) => return Err(e.to_string()),
        };

        Ok(DirWalk {
            path_stack: Vec::from([dir_entries]),
        })
    }

    pub async fn next_entry(&mut self) -> Option<PathBuf> {
        while let Some(mut dir_entries) = self.path_stack.pop() {
            while let Ok(entry_attempt) = dir_entries.next_entry().await {
                if let Some(entry) = entry_attempt {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        self.path_stack.push(dir_entries);

                        return Some(entry_path);
                    }

                    if entry_path.is_dir() {
                        self.path_stack.push(dir_entries);

                        let next_dir_entries = match read_dir(&entry_path).await {
                            Ok(rd) => rd,
                            Err(e) => return None,
                        };

                        self.path_stack.push(next_dir_entries);

                        return Some(entry_path);
                    }
                }

                break;
            }
        }

        None
    }
}

pub struct DirCopy {
    source_path: PathBuf,
    path_stack: Vec<(ReadDir, PathBuf)>,
}

impl DirCopy {
    pub async fn try_from_path(source_path: &PathBuf) -> Result<DirCopy, String> {
        let path_buf = match path::absolute(source_path) {
            Ok(p) => p,
            Err(e) => return Err(e.to_string()),
        };

        let dir_entries = match read_dir(&path_buf).await {
            Ok(rd) => rd,
            Err(e) => return Err(e.to_string()),
        };

        Ok(DirCopy {
            source_path: path_buf.clone(),
            path_stack: Vec::from([(dir_entries, path_buf)]),
        })
    }

    pub async fn next_entry(&mut self) -> Option<PathBuf> {
        while let Some((mut dir_entries, dir_path)) = self.path_stack.pop() {
            // pop path stack

            while let Ok(entry_attempt) = dir_entries.next_entry().await {
                if let Some(entry) = entry_attempt {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        self.path_stack.push((dir_entries, dir_path));

                        return Some(entry_path);
                    }

                    if entry_path.is_dir() {
                        self.path_stack.push((dir_entries, dir_path));

                        let next_dir_entries = match read_dir(&entry_path).await {
                            Ok(rd) => rd,
                            Err(e) => return None,
                        };

                        self.path_stack.push((next_dir_entries, entry_path.clone()));

                        return Some(entry_path);
                    }
                    // push dir_entries back onto the stack

                    // if file return path

                    //  if dir
                    //  read dir
                    //  add read dir and absolute back onto stack

                    // return absoluite dir path
                }

                break;
            }
        }

        None
    }
}
