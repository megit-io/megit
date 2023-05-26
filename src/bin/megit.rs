use clap::Parser;
use std::path::Path;
use git2::{Error,Commit,Repository, Branch};

/// MeGit
#[derive(Parser, Debug)]
#[command(author, version, about="MeGit", long_about = None)]
struct Args {
    /// Path to the repository to inspect
    #[arg(value_name="REPOSITORY_PATH", default_value=".")]
    path: String,
}

fn list_branches(_repo: &Repository) -> Option<Vec<Branch>> {
    None
}
//
// fn run(args: &Args) -> Result<(), git2::Error> {
//     let repo = Repository::open(".")?;
//     let mut revwalk = repo.revwalk()?;
//
//     let specs = args
//         .flag_not
//         .iter()
//         .map(|s| (s, true))
//         .chain(args.arg_spec.iter().map(|s| (s, false)))
//         .map(|(spec, hide)| {
//             if spec.starts_with('^') {
//                 (&spec[1..], !hide)
//             } else {
//                 (&spec[..], hide)
//             }
//         });
//     for (spec, hide) in specs {
//         let id = if spec.contains("..") {
//             let revspec = repo.revparse(spec)?;
//             if revspec.mode().contains(git2::RevparseMode::MERGE_BASE) {
//                 return Err(Error::from_str("merge bases not implemented"));
//             }
//             push(&mut revwalk, revspec.from().unwrap().id(), !hide)?;
//             revspec.to().unwrap().id()
//         } else {
//             repo.revparse_single(spec)?.id()
//         };
//         push(&mut revwalk, id, hide)?;
//     }
//
//     for id in revwalk {
//         let id = id?;
//         println!("{}", id);
//     }
//     Ok(())
// }
//
// fn push(revwalk: &mut Revwalk, id: Oid, hide: bool) -> Result<(), Error> {
//     if hide {
//         revwalk.hide(id)
//     } else {
//         revwalk.push(id)
//     }
// }


fn print_commit(commit: &Commit) {
    println!("commit {}", commit.id());

    if commit.parents().len() > 1 {
        print!("Merge:");
        for id in commit.parent_ids() {
            print!(" {:.8}", id);
        }
        println!();
    }

    let author = commit.author();
    println!("Author: {}", author);

    for line in String::from_utf8_lossy(commit.message_bytes()).lines() {
        println!("    {}", line);
    }
    println!();
}

fn do_shit() -> Result<(), Error> {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if !path.is_dir() {
        panic!("Not a directory!");
    }

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    // for id in revwalk {
    //     let id = id?;
    //     println!("{}", id);
    // }
    //
    // for _something in revwalk {
    //     println!("lalala");
    // }
    let commits = revwalk
        .filter_map(|id| {
            let id = id.unwrap();
            let commit = repo.find_commit(id);
            Some(commit)
        });

    for commit in commits {
        let commit = commit?;
        print_commit(&commit);

        // {
        //     Ok(m) => println!("{}", m),
        //     Err(E) => println!("ARAR")
        // }
    }

    list_branches(&repo);

    Ok(())

}

fn main(){
    do_shit().expect("OR ELSE");
}

