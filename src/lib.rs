// take a directory

// recursively copy files and directories in target directory
// .css
// .js

use std::path;
use std::path::PathBuf;
use tokio::fs::{read_dir, ReadDir};

// depth first
// can have max depth

async fn get_pathbufs_from_dir(dir_entries: &mut ReadDir) -> Vec<PathBuf> {
    let mut entries = Vec::new();

    while let Ok(entry_attempt) = dir_entries.next_entry().await {
        if let Some(entry) = entry_attempt {
            entries.push(entry.path());
            continue;
        }

        break;
    }

    entries
}

struct DirStackBit {
    entries: Vec<PathBuf>,
    index: usize,
}

impl DirStackBit {
    pub fn next(&mut self) -> Option<PathBuf> {
        if let Some(pb) = self.entries.get(self.index) {
            self.index += 1;
            return Some(pb.clone());
        }

        None
    }
}

pub struct DirWalk {
    path_stack: Vec<DirStackBit>,
}

impl DirWalk {
    pub async fn try_from_path(source_path: &PathBuf) -> Result<DirWalk, String> {
        let path_buf = match path::absolute(source_path) {
            Ok(p) => p,
            Err(e) => return Err(e.to_string()),
        };

        let dir_entries = match read_dir(&path_buf).await {
            Ok(mut rd) => get_pathbufs_from_dir(&mut rd).await,
            Err(e) => return Err(e.to_string()),
        };

        Ok(DirWalk {
            path_stack: Vec::from([DirStackBit {
                entries: dir_entries,
                index: 0,
            }]),
        })
    }

    pub async fn next_entry(&mut self) -> Option<PathBuf> {
        while let Some(mut dir_entries) = self.path_stack.pop() {
            while let Some(entry) = dir_entries.next() {
                if entry.is_file() {
                    self.path_stack.push(dir_entries);

                    return Some(entry);
                }

                if entry.is_dir() {
                    self.path_stack.push(dir_entries);

                    let mut next_read_dir = match read_dir(&entry).await {
                        Ok(rd) => rd,
                        Err(e) => return None,
                    };

                    let entries = get_pathbufs_from_dir(&mut next_read_dir).await;

                    self.path_stack.push(DirStackBit {
                        entries: entries,
                        index: 0,
                    });

                    return Some(entry);
                }
            }
        }

        None
    }
}

pub struct DirCopy {
    source_path: PathBuf,
    dest_path: PathBuf,
    path_stack: Vec<ReadDir>,
}

// want to pair a destination path

// get tail after source_path, append to dest_path

impl DirCopy {
    pub async fn try_from_path(
        source_path: &PathBuf,
        dest_path: &PathBuf,
    ) -> Result<DirCopy, String> {
        let source_path_buf = match path::absolute(source_path) {
            Ok(p) => p,
            Err(e) => return Err(e.to_string()),
        };

        let dest_path_buf = match path::absolute(dest_path) {
            Ok(p) => p,
            Err(e) => return Err(e.to_string()),
        };

        let dir_entries = match read_dir(&source_path_buf).await {
            Ok(rd) => rd,
            Err(e) => return Err(e.to_string()),
        };

        Ok(DirCopy {
            source_path: source_path_buf,
            dest_path: dest_path_buf,
            path_stack: Vec::from([dir_entries]),
        })
    }

    pub async fn next_entry(&mut self) -> Option<(PathBuf, PathBuf)> {
        while let Some(mut dir_entries) = self.path_stack.pop() {
            // pop path stack

            while let Ok(entry_attempt) = dir_entries.next_entry().await {
                if let Some(entry) = entry_attempt {
                    let entry_path = entry.path();

                    // strip prefix, join dest_path
                    let suffix = match entry_path.strip_prefix(&self.source_path) {
                        Ok(p) => p,
                        _ => continue,
                    };

                    let target_path = self.dest_path.join(suffix);

                    if entry_path.is_file() {
                        self.path_stack.push(dir_entries);

                        return Some((entry_path, target_path));
                    }

                    if entry_path.is_dir() {
                        self.path_stack.push(dir_entries);

                        let next_dir_entries = match read_dir(&entry_path).await {
                            Ok(rd) => rd,
                            Err(e) => return None,
                        };

                        self.path_stack.push(next_dir_entries);

                        return Some((entry_path, target_path));
                    }
                }

                break;
            }
        }

        None
    }
}
