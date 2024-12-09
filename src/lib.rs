// take a directory

// recursively copy files and directories in target directory
// .css
// .js

use std::path;
use std::path::PathBuf;
use tokio::fs::{copy, create_dir_all, read_dir, ReadDir};

// depth first
// can have max depth

/*
    one stack, stack of iterators
*/

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

        Ok(DirCopy {
            source_path: path_buf,
            path_stack: Vec::new(),
        })
    }

    pub async fn next_entry(&mut self) -> Option<PathBuf> {
        while let Some((mut dir_entries, dir_path)) = self.path_stack.pop() {
            // pop path stack

            while let Ok(entry_attempt) = dir_entries.next_entry().await {
                if let Some(entry) = entry_attempt {
                    let entry_path = entry.path();
                    println!("{:?}", &entry_path);

                    // push dir_entries back onto the stack

                    // if file return path

                    //  if dir
                    //  read dir
                    //  add read dir and absolute back onto stack

                    // return absoluite dir path
                }
            }
        }

        None
    }
}

pub async fn generate_assets(
    source_path: PathBuf,
    dest_path: PathBuf,
    _top_level_ext: &str,
) -> Result<(), String> {
    let source_abs = match path::absolute(&source_path) {
        Ok(p) => p,
        Err(e) => return Err(e.to_string()),
    };

    if source_abs.is_file() {
        return Err("found a file in dir stack".to_string());
    }

    let dest_abs = match path::absolute(&dest_path) {
        Ok(p) => p,
        Err(e) => return Err(e.to_string()),
    };

    if dest_abs.is_file() {
        return Err("found a file in dir stack".to_string());
    }

    let mut source_stack = Vec::<PathBuf>::from([source_abs]);
    let mut dest_stack = Vec::<PathBuf>::from([dest_abs]);

    // while theyre are source paths left
    while source_stack.len() != 0 {
        let curr_source_stack = source_stack;
        source_stack = Vec::<PathBuf>::new();

        let curr_dest_stack = dest_stack;
        dest_stack = Vec::<PathBuf>::new();

        let mut curr_itr = curr_source_stack.iter();
        let mut dest_itr = curr_dest_stack.iter();

        // iterate across source paths
        while let (Some(src_path), Some(dst_path)) = (curr_itr.next(), dest_itr.next()) {
            let mut dir_iter = match read_dir(&src_path).await {
                Ok(ditr) => ditr,
                Err(e) => return Err(e.to_string()),
            };

            // then iterate across entries in source path
            while let Ok(entry_attempt) = dir_iter.next_entry().await {
                if let Some(entry) = entry_attempt {
                    let source_entry = src_path.join(entry.path());
                    let dest_entry = dst_path.join(entry.path());

                    if source_entry.is_dir() {
                        if let Err(e) = create_dir_all(&dest_entry).await {
                            return Err(e.to_string());
                        }

                        source_stack.push(source_entry.clone());
                        dest_stack.push(dest_entry.clone());
                    }

                    if source_entry.is_file() {
                        // check extension
                        // if let Some(ext) = source_entry.extension() {
                        //     if ext == top_level_ext {
                        //         let _ = copy(source_entry, dest_entry).await;
                        //     }
                        // }

                        // copy file to location
                        let _ = copy(source_entry, dest_entry).await;
                    }
                }
            }
        }
    }

    Ok(())
}
