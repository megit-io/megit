use clap::Parser;
use megit::repo::repo_if_valid_path;
use megit::commit::MeCommit;

/// MeGit
#[derive(Parser, Debug)]
#[command(author, version, about = "MeGit", long_about = None)]
struct Args {
    /// Path to the repository to inspect
    #[arg(
    short = 'p',
    long = "path",
    help = "Path to the repository to check, if not set, will use PWD",
    value_name = "REPOSITORY_PATH",
    default_value = ".",
    required = false
    )]
    path: String,
    /// Name or email to find commits for
    #[arg(
    short = 'w',
    long = "who",
    value_name = "NAME_OR_EMAIL",
    help = "The name or email to find commits for. If none is specified, this is taken from your git config",
    required = false,
    default_value = None
    )]
    name_or_email: Option<String>,
}

fn main() {
    // TODO: NO_COLOR https://no-color.org/
    let args = Args::parse();

    let repo = repo_if_valid_path(&args.path).unwrap();

    let name_or_email = match args.name_or_email {
        Some(name_or_email) => name_or_email,
        None => repo.author_email().unwrap()
    };
    println!("Searching for {}", name_or_email);

    let mut prev: Option<&MeCommit>;
    let commits = repo.list_commits().unwrap();

    let mut total_change: usize = 0;

    for (i, commit) in commits.iter().enumerate() {
        if commit.get_author_email().unwrap() == name_or_email
            || commit.get_author_name().unwrap() == name_or_email {
            println!("Found commit {}", commit.sha);
            if i > 0 {
                prev = commits.get(i-1);
                println!("Previous is {}", prev.unwrap().sha);

                let diff = repo.diff(prev.unwrap(), commit).unwrap();

                let new_lines = diff.new_lines().unwrap();
                println!("Added {} lines", new_lines);
                total_change += new_lines;

                let removed_lines = diff.removed_lines().unwrap();
                println!("Removed {} lines", removed_lines);
                total_change -= removed_lines;
            }
        }
    }

    println!("Total change in lines: {}", total_change);
}
