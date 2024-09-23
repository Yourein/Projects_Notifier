mod adapter;
mod redis_wrapper;

use std::env;
use redis_wrapper::RedisWrapper;
use github::client::Client;
use slack::webhook::Webhook;
use slack::post::Post;

fn main() {
    // This app panics when required variables are not found or failed to connect to redis server

    let slack_end_point = env::var("MAIGO_SLACK_URL").unwrap();
    let github_token = env::var("MAIGO_GITHUB_TOKEN").unwrap();
    let redis_client = RedisWrapper::new("redis:127.0.0.1").unwrap();
    let slack = Webhook::new(slack_end_point);
    let gc = Client::new(&github_token);
}

fn post_task_to_slack<'a>(client: Webhook, post: Post<'a>) {
    match client.post(post) {
        Ok(_) => {
            println!{"Post Success!"}
        }
        Err(_) => {
            println!{"Post Failed"}
        }
    }
}
