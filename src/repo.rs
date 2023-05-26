/*
 TODO: What is the conventional way to add comments to things in Rust
*/
use std::path::Path;
use git2::Repository; //, Commit, Branch};

// mod repo_priv {
//     use git2::Repository; //, Commit, Branch};

pub struct Repo {
    inner: Repository,
}

// pub struct Branch {}
// pub struct Commit {}

    // impl Repo {
    //     // fn list_commits(&self, branch: &Branch) -> Vec<Commit> {
    //     // }
    //     //
    //     // fn list_branches(&self) -> Vec<Branch> {
    //     // }
    // }
// }

// use repo_priv::Repo;

fn is_git_repository(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }
    // path.read_dir()
    true
}

fn repo_if_valid_path(path_str: &String) -> Result<Repo, String> {
    let path = Path::new(path_str);
    if !is_git_repository(path) {
        return Err(format!("The path {path_str} is not a directory containing a .git repository"))
    }
    Ok(Repo{
        inner: Repository::open(path).expect("FFSS")
    })
}