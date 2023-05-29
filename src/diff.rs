use git2::{Diff, Repository, Error, DiffFormat, DiffLine, DiffHunk, DiffDelta, DiffLineType};
use crate::commit::MeCommit;

pub struct MeDiff<'repo> {
    inner: Diff<'repo>
}


fn count_lines(
    _delta: DiffDelta,
    _hunk: Option<DiffHunk>,
    line: DiffLine,
    line_type: DiffLineType,
    counter: &mut usize) -> bool {
    if line_type == line.origin_value() {
        *counter += 1;
    }
    true
}

impl<'repo> MeDiff<'repo> {
    pub fn new(repo: &'repo Repository, from_commit: &'repo MeCommit, to_commit: &'repo MeCommit) -> Result<MeDiff<'repo>, Error> {
        let tree_left = from_commit.tree()?;
        let tree_right = to_commit.tree()?;
        let diff = repo.diff_tree_to_tree(Some(&tree_left), Some(&tree_right), None)?;
        let diff = MeDiff { inner: diff };
        Ok(diff)
    }

    pub fn new_lines(&self) -> Result<usize, Error> {
        self.count_lines(DiffLineType::Addition)
    }

    pub fn removed_lines(&self) -> Result<usize, Error> {
        self.count_lines(DiffLineType::Deletion)
    }

    fn count_lines(&self, line_type: DiffLineType) -> Result<usize,Error> {
        // need a u32 to be able to accumulate ; dont want to faff converting to a usize after
        let format = DiffFormat::Patch;
        let mut counter: usize = 0;

        // bit hacky to abuse the print callback this way but didn't see an easier
        // option to iterate over all diffs and all lines...
        self.inner.print(format, |d, h, l|
            count_lines(d, h, l, line_type, &mut counter))?;

        // make it immutable now
        let count = counter;
        Ok(count)
    }
}


/* ----------------------------------------------------------------
        TESTS!
   ----------------------------------------------------------------*/
//
// #[cfg(test)]
// mod tests {
//     use crate::repo::{clone_commit, repo_if_valid_path};
//
//     #[test]
//     fn test_basic_diff() {
//         let repo = repo_if_valid_path(".").expect("Should find a repository in the present directory");
//         let commits = repo.list_commits().expect("Should have a list of commits in the repository");
//
//         // sanity check we have at least two commits to compare
//         assert!(commits.len() > 1);
//
//         let commit1 = clone_commit(&repo, commits.get(0).unwrap());
//
//
//         // with thanks to https://stackoverflow.com/a/29509257 :
//         let (commit1, commit2) = match &commits[..] {
//             [first, second, ..] => (first, second),
//             _ => unreachable!(),
//         };
//
//         let diff = repo.diff(commit1, commit2).expect("Should create a diff of two commits");
//
//         let count = diff.new_lines().unwrap();
//         assert!(count > 0);
//
//         let count = diff.removed_lines().unwrap();
//         assert!(count > 0);
//     }
// }
