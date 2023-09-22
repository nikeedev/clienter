use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Profile {
    id:      u32,
    status:  String,
    bio:     String,
    country: String
}

#[derive(Debug, Deserialize, Serialize)]
struct History {
    joined: String
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiInfo {
	id:          u32,
	username:    String,
	scratchteam: bool,
	history: History,

	profile: Profile
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = "nikeedev";

    let resp = reqwest::get(format!("https://api.scratch.mit.edu/users/{username}"))
        .await?
        .text()
        .await?;
    let res: ApiInfo = serde_json::from_str(&resp).unwrap();
    println!("{:#?}", res);
    Ok(())
}
