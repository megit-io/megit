use git2::{Diff, Repository, Tree, Error, DiffLineType, DiffFormat, DiffLine, DiffHunk, DiffDelta};

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
    pub fn new(repo: &'repo Repository, tree_left: Tree<'repo>, tree_right: Tree<'repo>) -> Result<MeDiff<'repo>, Error> {
        let diff = repo.diff_tree_to_tree(Some(&tree_left), Some(&tree_right), None)?;
        let diff = MeDiff { inner: diff };
        Ok(diff)
    }

    pub fn count_lines(&self, line_type: DiffLineType) -> Result<usize,Error> {
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

#[cfg(test)]
mod tests {
    use git2::DiffLineType;
    use crate::repo::repo_if_valid_path;

    #[test]
    fn test_basic_diff() {
        let repo = repo_if_valid_path(".").expect("Should find a repository in the present directory");
        let commits = repo.list_commits().expect("Should have a list of commits in the repository");

        // sanity check we have at least two commits to compare
        assert!(commits.len() > 1);

        // with thanks to https://stackoverflow.com/a/29509257 :
        let (commit1, commit2) = match &commits[..] {
            [first, second, ..] => (first, second),
            _ => unreachable!(),
        };

        let diff = repo.diff(commit1, commit2).expect("Should create a diff of two commits");

        let count = diff.count_lines(DiffLineType::Addition).unwrap();
        assert!(count > 0);

        let count = diff.count_lines(DiffLineType::Deletion).unwrap();
        assert!(count > 0);
    }
}
