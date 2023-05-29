use git2::{Commit, Error, Tree};
use chrono::{DateTime, Local, TimeZone};


pub struct MeCommit<'repo> {
    inner: Commit<'repo>,
    pub sha: String
}

impl<'repo> MeCommit<'repo> {
    pub fn new(commit: Commit) -> MeCommit {
        let sha = commit.id().to_string();
        MeCommit {
            sha,
            inner: commit
        }
    }

    pub fn get_author_email(&self) -> Result<String, Error> {
        let author = self.inner.author();
        let author = author.email().unwrap().to_string();
        Ok(author)
    }

    pub fn get_author_name(&self) -> Result<String, Error> {
        let author = self.inner.author();
        let author = author.name().unwrap().to_string();
        Ok(author)
    }

    pub fn datetime(&self) -> DateTime<Local> {
        let time = self.inner.time().seconds();
        let timestamp = chrono::NaiveDateTime::from_timestamp_opt(time, 0).unwrap();
        Local.from_utc_datetime(&timestamp)
    }

    pub fn tree(&self) -> Result<Tree, Error> {
        return self.inner.tree();
    }
}
