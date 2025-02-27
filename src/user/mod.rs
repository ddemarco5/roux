//! # User
//! A read-only module to read data from for a specific user.
//!
//! # Usage
//! ```rust
//! use roux::User;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!     let user = User::new("beanpup_py");
//!     // Now you are able to:
//!
//!     // Get overview
//!     let overview = user.overview().await;
//!
//!     // Get submitted posts.
//!     let submitted = user.submitted().await;
//!
//!     // Get comments.
//!     let comments = user.comments().await;
//! }
//! ```

extern crate reqwest;
extern crate serde_json;

use crate::util::RouxError;
use reqwest::Client;

pub mod responses;
use crate::subreddit::responses::{Submissions, SubredditComments};
use responses::Overview;

/// User.
pub struct User {
    /// User's name.
    pub user: String,
    client: Client,
}

impl User {
    /// Create a new `User` instance.
    pub fn new(user: &str) -> User {
        User {
            user: user.to_owned(),
            client: Client::new(),
        }
    }
    
    /// Creat a new user instance using a provided user agent string
    pub fn new_with_agent(user: &str, agent: &str) -> User {
        println!("user agent is: {}", agent);
        let client = reqwest::Client::builder()
                        .user_agent(agent)
                        .build().expect("Error building reqwest client");
        User {
            user: user.to_owned(),
            client: client,
        }
    }

    /// Get user's overview.
    pub async fn overview(&self) -> Result<Overview, RouxError> {
        Ok(self
            .client
            .get(&format!(
                "https://www.reddit.com/user/{}/overview/.json",
                self.user
            ))
            .send()
            .await?
            .json::<Overview>()
            .await?)
    }

    /// Get user's submitted posts.
    pub async fn submitted(&self) -> Result<Submissions, RouxError> {
        println!("client info\n{:?}", self.client.get("https://www.reddit.com/user/Hentoota-Kitty/submitted.json").header(reqwest::header::USER_AGENT, "test"));
        let response = self
            .client
            .get(&format!(
                "https://www.reddit.com/user/{}/submitted/.json",
                self.user
            ))
            //.header(reqwest::header::USER_AGENT, "test")
            .send()
            .await?;

        println!("Response status: {:?}", response.status());
        println!("Reponse headers:\n{:?}", response.headers());
        Ok(response.json::<Submissions>().await?)
        /*
        Ok(self
            .client
            .get(&format!(
                "https://www.reddit.com/user/{}/submitted/.json",
                self.user
            ))
            //.header(reqwest::header::USER_AGENT, "test")
            .send()
            .await?
            .json::<Submissions>()
            .await?)
        */
    }

    /// Get user's submitted comments.
    pub async fn comments(&self) -> Result<SubredditComments, RouxError> {
        Ok(self
            .client
            .get(&format!(
                "https://www.reddit.com/user/{}/comments/.json",
                self.user
            ))
            .send()
            .await?
            .json::<SubredditComments>()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use tokio;

    #[tokio::test]
    async fn test_no_auth() {
        let user = User::new("beneater");

        // Test overview
        let overview = user.overview().await;
        assert!(overview.is_ok());

        // Test submitted
        let submitted = user.submitted().await;
        assert!(submitted.is_ok());

        // Test comments
        let comments = user.comments().await;
        assert!(comments.is_ok());
    }
}
