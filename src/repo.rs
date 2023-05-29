use git2::{BranchType, Error, ErrorClass, ErrorCode, Repository, Sort};
use std::path::Path;
use crate::branch::MeBranch;
use crate::commit::MeCommit;
use crate::diff::MeDiff;

/// Represents a Git repository.
pub struct MeRepo {
    inner: Repository,
}

impl MeRepo {
    /// Iterates over all local branches in the repository and returns a wrapper for each branch.
    pub fn list_branches(&self) -> Result<Vec<MeBranch>, Error> {
        let mut branches = Vec::new();
        let repo = &self.inner;

        for branch in repo.branches(Some(BranchType::Local))? {
            let (branch, _) = branch?;
            if let Some(branch_name) = branch.name()?.map(|name| name.to_string()) {
                branches.push(MeBranch { name: branch_name });
            }
        }

        Ok(branches)
    }

    /// Iterates over all commits in the repository.
    pub fn list_commits(&self) -> Result<Vec<MeCommit>, Error> {
        let repo = &self.inner;
        let mut revwalk = repo.revwalk()?;

        revwalk.set_sorting(Sort::REVERSE)?;
        revwalk.push_head()?;

        let mut commits = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = MeCommit::new(repo.find_commit(oid)?);
            commits.push(commit);
        }

        Ok(commits)
    }

    pub fn diff<'repo>(&'repo self, from_commit: &'repo MeCommit, to_commit: &'repo MeCommit) -> Result<MeDiff<'repo>, Error> {
        let tree_left = from_commit.tree()?;
        let tree_right = to_commit.tree()?;
        let diff = MeDiff::new(&self.inner, tree_left, tree_right)?;
        Ok(diff)
    }

    pub fn author_email(&self) -> Result<String, Error> {
        let conf = self.inner.config()?;

        let result = match conf.get_entry("user.email") {
            Ok(entry) => {
                let email = entry.value().unwrap();
                Ok(email.to_string())
            }
            Err(err) => Err(err)
        };
        result
    }
}


/// Checks if a directory contains a Git repository.
fn is_git_repository_dir(path: &Path) -> bool {
    let git_directory = path.join(".git");
    path.is_dir() && git_directory.is_dir()
}

/// Creates a `Repo` object if the given path is a valid Git repository.
pub fn repo_if_valid_path(path: &str) -> Result<MeRepo, Error> {
    let no_repo = Error::new(ErrorCode::NotFound, ErrorClass::Invalid,
                             format!("The path '{}' is not a valid Git repository", &path));

    let path = Path::new(path);
    if !is_git_repository_dir(path) {
        return Err(no_repo);
    }

    let repository = Repository::open(path)?;
    return Ok(MeRepo { inner: repository });
}

/* ----------------------------------------------------------------
        TESTS!
   ----------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::{is_git_repository_dir, repo_if_valid_path};

    #[test]
    fn test_email_from_config() {
        let repo = repo_if_valid_path(".").unwrap();
        let email = repo.author_email().unwrap();
        assert!(!email.is_empty());
    }

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
        for branch in repo.list_branches().unwrap() {
            assert!(!branch.name.is_empty());
        }
        for commit in repo.list_commits().unwrap() {
            assert!(!commit.get_author_email().unwrap().is_empty())
        }
    }

    #[test]
    fn test_commit_ordering() {
        let repo = repo_if_valid_path(".").unwrap();
        let commits = repo.list_commits().unwrap();

        for (prev_commit, curr_commit) in commits.iter().zip(commits.iter().skip(1)) {
            let prev_datetime = prev_commit.datetime();
            let curr_datetime = curr_commit.datetime();
            println!("{} : {}", prev_datetime, curr_datetime);

            // Assert that the current commit is more recent than the previous commit
            assert!(curr_datetime > prev_datetime);
        }
    }
}
