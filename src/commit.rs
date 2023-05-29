use git2::{Commit, Error, ErrorClass, ErrorCode, Oid, Tree};
use chrono::{DateTime, Local, TimeZone};


pub struct MeCommit<'repo> {
    inner: Commit<'repo>,
}

impl<'repo> MeCommit<'repo> {

    pub fn new(commit: Commit) -> MeCommit {
        MeCommit {
            inner: commit,
        }
    }

    pub fn oid(&self) -> Oid {
        self.inner.id()
    }

    pub fn sha(&self) -> String {
        self.oid().to_string()
    }

    pub fn get_author_email(&self) -> Result<String, Error> {
        let author = self.inner.author();
        match author.email() {
            Some(email) => Ok(email.to_string()),
            None => Err(Error::new(ErrorCode::Invalid, ErrorClass::Config,
                                   "Could not fetch author email"))
        }
    }

    pub fn get_author_name(&self) -> Result<String, Error> {
        let author = self.inner.author();
        match author.name() {
            Some(name) => Ok(name.to_string()),
            None => Err(Error::new(ErrorCode::Invalid, ErrorClass::Config,
                                   "Could not fetch author name"))
        }
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
