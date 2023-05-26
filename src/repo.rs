use git2::{Error, ErrorClass, ErrorCode, Repository, Commit, BranchType};
use std::path::Path;

/// Represents a Git repository.
pub struct Repo {
    inner: Repository,
}

/// Information about a Git branch.
struct BranchInfo {
    name: String,
}

impl Repo {
    /// Iterates over all local branches in the repository and returns information about each branch.
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

    /// Iterates over all commits in the repository.
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

/// Checks if a directory contains a Git repository.
fn is_git_repository_dir(path: &Path) -> bool {
    let git_directory = path.join(".git");
    path.is_dir() && git_directory.is_dir()
}

/// Creates a `Repo` object if the given path is a valid Git repository.
fn repo_if_valid_path(path: &str) -> Result<Repo, Error> {
    let path_str = path.to_owned();
    let path = Path::new(path);
    if is_git_repository_dir(path) {
        let repository = Repository::open(path)?;
        return Ok(Repo { inner: repository });
    }
    Err(Error::new(ErrorCode::NotFound, ErrorClass::Invalid, format!("The path '{}' is not a valid Git repository", path_str)))
}

/* ----------------------------------------------------------------
        TESTS!
   ----------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::{is_git_repository_dir, repo_if_valid_path};

    #[test]
    fn test_git_repo_detection() {
        // Note: These paths assume running `cargo test` from the root of the Git repository
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
