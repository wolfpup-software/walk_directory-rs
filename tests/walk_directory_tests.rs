use std::path::PathBuf;
use walk_directory::DirCopy;

#[tokio::test]
async fn walk_the_dog() {
    let path_buf = PathBuf::from("./test_dir");

    let mut dir_copy = match DirCopy::try_from_path(&path_buf).await {
        Ok(d) => d,
        Err(e) => return assert_eq!("dir copy failed", "dir_copy failed"),
    };

    while let Some(path_buf) = dir_copy.next_entry().await {}
}
