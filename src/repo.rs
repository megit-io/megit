/*
 TODO: What is the conventional way to add comments to things in Rust
*/
use git2::{Error, ErrorClass, ErrorCode, Repository};
use std::path::Path;

pub struct Repo {
    inner: Repository,
}


fn is_git_repository_dir(path: &Path) -> bool {
    let git_directory = path.join(".git");
    path.is_dir() && git_directory.is_dir()
}

fn repo_if_valid_path(path: &str) -> Result<Repo, Error> {
    let path_str = path.clone();
    let path = Path::new(path);
    if is_git_repository_dir(path) {
        let repository = Repository::open(path)?;
        return Ok(Repo { inner: repository });
    }
    Err(Error::new(ErrorCode::NotFound, ErrorClass::Invalid, format!("The path {path_str} is not a valid Git repository")))
}


/* ----------------------------------------------------------------
        TESTS!
   ----------------------------------------------------------------*/

mod tests {
    use std::path::Path;
    use crate::repo::{is_git_repository_dir, repo_if_valid_path};

    #[test]
    fn test_git_repo_detection() {
        /* Note: this is making the assumption that we are running `cargo test` from the root
           of the megit repository, so these paths are not relative to this repo.rs file */
        assert!(is_git_repository_dir(Path::new(".")));
        assert!(!is_git_repository_dir(Path::new("./src")));
    }

    #[test]
    fn test_get_repo() {
        // Just try making a repository object
        assert!(repo_if_valid_path(".").is_ok());
    }
}
