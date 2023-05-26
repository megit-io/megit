use clap::Parser;
use megit::repo::repo_if_valid_path;


/// MeGit
#[derive(Parser, Debug)]
#[command(author, version, about="MeGit", long_about = None)]
struct Args {
    /// Path to the repository to inspect
    #[arg(short='p', long="path", help="Path to the repository to check, if not set, will use PWD",
          value_name="REPOSITORY_PATH", default_value=".", required=false)]
    path: String,
    /// Username to find commits for
    #[arg(value_name="USERNANME", required=true)]
    username: String
}

fn main() {
    let args = Args::parse();
    println!("{}", args.username);

    let repo = repo_if_valid_path(&args.path).unwrap();
    for branch in repo.iter_branches().unwrap() {
        println!("{}", branch.name)
    }
    for commit in repo.iter_commits().unwrap() {
        println!("{}", commit.id())
    }
}
