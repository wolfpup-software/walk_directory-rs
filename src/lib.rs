use std::path;
use std::path::PathBuf;
use tokio::fs::{read_dir, ReadDir};

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
    pub async fn try_from(path_buf: &PathBuf) -> Result<DirStackBit, String> {
        let path_buf = match path::absolute(path_buf) {
            Ok(p) => p,
            Err(e) => return Err(e.to_string()),
        };

        let dir_entries = match read_dir(path_buf).await {
            Ok(mut rd) => get_pathbufs_from_dir(&mut rd).await,
            Err(e) => return Err(e.to_string()),
        };

        Ok(DirStackBit {
            entries: dir_entries,
            index: 0,
        })
    }

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

        let dir_stack_bit = match DirStackBit::try_from(&path_buf).await {
            Ok(de) => de,
            Err(e) => return Err(e),
        };

        Ok(DirWalk {
            path_stack: Vec::from([dir_stack_bit]),
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

                    let mut next_stack_bit = match DirStackBit::try_from(&entry).await {
                        Ok(rd) => rd,
                        Err(e) => return None,
                    };

                    self.path_stack.push(next_stack_bit);

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
    path_stack: Vec<DirStackBit>,
}

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

        let dir_stack_bit = match DirStackBit::try_from(&source_path).await {
            Ok(de) => de,
            Err(e) => return Err(e),
        };

        Ok(DirCopy {
            source_path: source_path_buf,
            dest_path: dest_path_buf,
            path_stack: Vec::from([dir_stack_bit]),
        })
    }

    pub async fn next_entry(&mut self) -> Option<(PathBuf, PathBuf)> {
        while let Some(mut dir_entries) = self.path_stack.pop() {
            while let Some(entry_path) = dir_entries.next() {
                let suffix = match entry_path.strip_prefix(&self.source_path) {
                    Ok(p) => p,
                    // it is assumed that DirCopy only works with absolute filepaths
                    Err(e) => continue,
                };

                let target_path = self.dest_path.join(suffix);

                if entry_path.is_file() {
                    self.path_stack.push(dir_entries);

                    return Some((entry_path, target_path));
                }

                if entry_path.is_dir() {
                    self.path_stack.push(dir_entries);

                    let next_dir_stack_bit = match DirStackBit::try_from(&entry_path).await {
                        Ok(de) => de,
                        Err(e) => return None,
                    };

                    self.path_stack.push(next_dir_stack_bit);

                    return Some((entry_path, target_path));
                }
            }
        }

        None
    }
}
