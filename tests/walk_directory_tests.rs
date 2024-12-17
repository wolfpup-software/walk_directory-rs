use std::path::PathBuf;
use walk_directory::{DirCopy, DirWalk};

#[tokio::test]
async fn walk_the_dog() {
    let path_buf = PathBuf::from("./test_dir");

    let mut dir_copy = match DirWalk::try_from_path(&path_buf).await {
        Ok(d) => d,
        Err(e) => return assert_eq!("dir copy failed", "dir_copy failed"),
    };

    while let Some(path_buf) = dir_copy.next_entry().await {
        // println!("{:?}", path_buf);
    }
}

#[tokio::test]
async fn walk_the_dog_doppleganger() {
    let source_path_buf = PathBuf::from("./test_dir");
    let dest_path_buf = PathBuf::from("./target_test_dir");

    let mut dir_copy = match DirCopy::try_from_path(&source_path_buf, &dest_path_buf).await {
        Ok(d) => d,
        Err(e) => return assert_eq!("dir copy failed", "dir_copy failed"),
    };

    while let Some((path_buf, target_path_buf)) = dir_copy.next_entry().await {
        // println!("{:?}", path_buf);
        // println!("{:?}", target_path_buf);
    }
}
