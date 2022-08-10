use clap::Parser;
use std::{collections::HashMap, io::{stdout, Write}};

mod githubapi;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// github username
    #[clap(short, long, value_parser)]
    username: String,
 
    /// Number of times to greet
    #[clap(short, long, value_parser)]
    token: String,
}

struct FriendMap {
    cnt: u32,
    // repo_blacklists: Vec<String>,
    f: HashMap<String, u32>,
}

 
fn findfriends(username:String, github_token: String) -> FriendMap {

    let user_starred_infos = githubapi::get_starred_repos(username, &github_token);

    let mut cnt:u32 = 0;
    // let mut repo_blacklists =  <Vec<String>>::new();
    let mut f = HashMap::new();

    let cnt_end = user_starred_infos.len();
    for repo in user_starred_infos {
        cnt = cnt + 1;
        print!("\rcnt: {}/{} ", cnt, cnt_end);
        // page limit 400, 400 * 100
        if repo.stargazers_count > 40000 {
            // repo_blacklists.push(repo.full_name);
            continue;
        }

        let _ = stdout().flush();
        let users = githubapi::get_starred_users(repo.full_name, repo.stargazers_count, &github_token);

        for user in users {
            let count = f.entry(user.login).or_insert(0);
            *count += 1;
        }
    }
    let friendmap = FriendMap {
        cnt,
        // repo_blacklists,
        f
    };

    friendmap
}


fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.username);
    let friendmap = findfriends(args.username, args.token);
    let mut sortedv: Vec<_> = friendmap.f.iter().collect();
    sortedv.sort_by(|x,y| x.1.cmp(&y.1));
    println!("cnt: {}", friendmap.cnt);
    for afriend in sortedv {
        if afriend.1 < &10 {
            continue;
        }
        println!("{}: {}", afriend.0, afriend.1);
    }
}
