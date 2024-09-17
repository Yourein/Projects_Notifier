use crate::post::Post;
use reqwest::blocking as reqwestBlocking;

pub struct Webhook {
    end_point: String
}

impl Webhook {
    pub fn new(end_point: String) -> Webhook {
        Webhook {
            end_point: end_point
        }
    }

    pub fn post(&self, post: Post) -> Result<(), String> {
        let Ok(json) = serde_json::to_string(&post) else {
            return Err("Failed to parse Post".to_string());
        };
        
        let res = reqwestBlocking::Client::new()
            .post(&self.end_point)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(json)
            .send();
        
        if res.is_ok() {
            if res.unwrap().status().is_success() {
                Ok(())
            } else {
                Err("Something Wrong!".to_string())
            }
        } else {
            Err("Send Error!".to_string())
        }
    }
}