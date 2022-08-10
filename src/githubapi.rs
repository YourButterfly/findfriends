use reqwest::header::{USER_AGENT, AUTHORIZATION};
use std::{io::{stdout, Write}, path::Path};
use std::fs;
use serde_derive::{Serialize, Deserialize};

const GITHUB_API_URL:&str =  "https://api.github.com";

const STARRED_REPOS_PATH:&str =  ".findfriends/starred_repos";
const STARGAZERS_PATH:&str =  ".findfriends/stargazers";
const GITHUB_PAGE_LIMIT:u64 = 400;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserStarredInfo {
    id: u64,
    pub full_name: String,
    html_url: String,
    pub stargazers_count: u64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub login: String,
}


// https://docs.github.com/cn/rest/activity/starring
pub fn get_starred_repos(username: String, github_token: &String) ->  Vec<UserStarredInfo> {
    let req_url:String = GITHUB_API_URL.to_string() +  format!("/users/{username}/starred").as_str();
    println!("{:#?}", &req_url);

    let workspace = Path::new(&STARRED_REPOS_PATH).join(Path::new(&username));
    let _ = fs::create_dir_all(&workspace);

    let client = reqwest::blocking::Client::new();

    let mut page:u32 = 1;
    let mut rslt = <Vec<UserStarredInfo>>::new();
    loop {
        let cur_page_path = Path::new(&workspace).join(format!("{page}.json"));
        let mut is_break = 0;

        if cur_page_path.exists() {
            let data = fs::read_to_string(cur_page_path)
                .expect("Unable to read file");
            let mut usi: Vec<UserStarredInfo> = serde_json::from_str(&data)
                .expect("JSON does not have correct format.");
            rslt.append(&mut usi);

        } else {
            let resp = client.get(&req_url)
            .header(USER_AGENT, "findfriends")
            .header(AUTHORIZATION, format!("token {github_token}"))
            .query(&[("page", format!("{page}"))])
            .query(&[("per_page", "100")])
            .send();

            print!("\rpage: {}", page);
            let _ = stdout().flush();
            is_break = match resp {
                Ok(r) => {
                    let _t = match r.status().is_success() {
                        true => {
                            let mut _a = r.json::<Vec<UserStarredInfo>>().expect("failed to parse response");

                            if _a.len() != 0 {
                                let usi = _a.clone();
                                rslt.append(&mut _a);
                                let _ = std::fs::write(cur_page_path,serde_json::to_string_pretty(&usi).unwrap());
                                0
                            } else {
                                1
                            }
                        },
                        false => 1,
                    };
                    _t
                }, 
                Err(_) => 1,
            };
        }

        if is_break == 1 {
            break;
        }

        page = page + 1;
    }
    rslt
}


pub fn get_starred_users(repo_fullname: String, stargazers_count:u64, github_token: &String) -> Vec<UserInfo> {
    let req_url:String = GITHUB_API_URL.to_string() +  format!("/repos/{repo_fullname}/stargazers").as_str();
    println!(" {}", repo_fullname);
    let workspace = Path::new(&STARGAZERS_PATH).join(Path::new(&repo_fullname));
    let _ = fs::create_dir_all(&workspace);

    let client = reqwest::blocking::Client::new();

    let mut page:u64 = 1;
    let mut rslt = <Vec<UserInfo>>::new();
    let page_end:u64= stargazers_count / 100 + 1;
    loop {
        let cur_page_path = Path::new(&workspace).join(format!("{page}.json"));
        let mut is_break = 0;
        if page > page_end || page > GITHUB_PAGE_LIMIT {
            break;
        }

        if cur_page_path.exists() {
            let data = fs::read_to_string(cur_page_path)
                .expect("Unable to read file");
            let mut usi: Vec<UserInfo> = serde_json::from_str(&data)
                .expect("JSON does not have correct format.");
            rslt.append(&mut usi);
            print!("\rpage: {}/{}", page, page_end);
            let _ = stdout().flush();
        } else {
            let resp = client.get(&req_url)
            .header(USER_AGENT, "findfriends")
            .header(AUTHORIZATION, format!("token {github_token}"))
            .query(&[("page", format!("{page}"))])
            .query(&[("per_page", "100")])
            .send();

            print!("\rpage: {}/{}", page, page_end);
            let _ = stdout().flush();
            is_break = match resp {
                Ok(r) => {
                    let _t = match r.status().is_success() {
                        true => {
                            let mut _a = r.json::<Vec<UserInfo>>().expect("failed to parse response");

                            if _a.len() != 0 {
                                let usi = _a.clone();
                                rslt.append(&mut _a);
                                let _ = std::fs::write(cur_page_path,serde_json::to_string_pretty(&usi).unwrap());
                                0
                            } else {
                                1
                            }
                        },
                        false => 1,
                    };
                    _t
                }, 
                Err(_) => 1,
            };
        }

        if is_break == 1 {
            break;
        }
        page = page + 1;
    }

    rslt
}

