use git2::{Commit, Error, Tree};
use chrono::{DateTime, Local, TimeZone};


pub struct MeCommit<'repo> {
    sha: String,
    inner: Commit<'repo>
}

impl<'repo> MeCommit<'repo> {

    pub fn new(commit: Commit) -> MeCommit {
        let sha = &commit.id().to_string();
        let sha = sha.clone();

        MeCommit {
            sha,
            inner: commit
        }
    }

    pub fn get_author(&self) -> Result<String, Error> {
        let author = self.inner.author().name().unwrap().to_string();
        Ok(author)
    }

    pub fn datetime(&self) -> DateTime<Local> {
        let time = self.inner.time().seconds();
        let timestamp = chrono::NaiveDateTime::from_timestamp_opt(time, 0).unwrap();
        Local.from_utc_datetime(&timestamp)
    }

    pub fn tree(&self) -> Result<Tree, Error> {
        return self.inner.tree()
    }
}

// pub struct MeDiff<'repo> {
//     inner: Diff<'repo>
// }
//
// impl<'repo> MeDiff<'repo> {
//
//     fn count_lines_by_type(&self, line_type: DiffLineType) -> usize {
//         let mut matching_lines: usize = 0;
//
//         // let mut opts = DiffOptions::new();
//
//
//         for delta in self.inner.deltas() {
//             for hunk in delta.new_hunks() {
//                 for line in hunk.lines() {
//                     if line.origin() == line_type {
//                         matching_lines += 1;
//                     }
//                 }
//             }
//         }
//
//         // self.inner.foreach(&mut |delta: DiffDelta, _progress: f64| {
//         //     let hunk = delta.new_hunk();
//         //     let lines = hunk.lines();
//         //
//         //     for line in lines {
//         //         if line.origin() == line_type {
//         //             matching_lines += 1;
//         //         }
//         //     }
//         //
//         //     true
//         // }, None, None, None)
//         //     .unwrap();
//
//         matching_lines
//     }
// }
