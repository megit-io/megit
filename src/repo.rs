/*
 TODO: What is the conventional way to add comments to things in Rust
*/
use git2::{Error, ErrorClass, ErrorCode, Repository, Commit, BranchType};
use std::path::Path;

pub struct Repo {
    inner: Repository,
}

struct BranchInfo {
    name: String,
}

impl Repo {

    fn iter_branches(&self) -> Result<Vec<BranchInfo>, Error> {
        let mut branches = Vec::new();
        let repo = &self.inner;

        for branch in repo.branches(Some(BranchType::Local))? {
            let (branch, _) = branch?;
            if let Some(branch_name) = branch.name()?.map(|name| name.to_string()) {
                branches.push(BranchInfo { name: branch_name });
            }
        }

        Ok(branches)
    }

    fn iter_commits(&self) -> Result<Vec<Commit>, Error> {
        let repo = &self.inner;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        let mut commits = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            commits.push(commit);
        }

        Ok(commits)
    }
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

    #[test]
    fn test_get_branches_and_get_commits() {
        let repo = repo_if_valid_path(".").unwrap();
        for branch in repo.iter_branches().unwrap() {
            assert!(!branch.name.is_empty());
        }
        for commit in repo.iter_commits().unwrap() {
            assert!(!commit.author().name().is_none())
        }
    }
}
