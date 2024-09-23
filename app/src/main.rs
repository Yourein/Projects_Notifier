mod adapter;
mod redis_wrapper;

use std::env;
use adapter::ProjectTaskShrink;
use redis_wrapper::RedisWrapper;
use github::client::Client;
use slack::webhook::Webhook;
use slack::post::Post;

fn main() {
    // This app panics when required variables are not found or failed to connect to redis server

    let slack_end_point = env::var("MAIGO_SLACK_URL").unwrap();
    let github_token = env::var("MAIGO_GITHUB_TOKEN").unwrap();
    let mut redis_client = RedisWrapper::new("redis:127.0.0.1").unwrap();
    let slack = Webhook::new(slack_end_point);
    let gc = Client::new(&github_token);

    initialize(&mut redis_client, &gc);
}

fn initialize(
    redis: &mut RedisWrapper,
    gc: &Client
) {
    let get_result = gc.get_project_tasks("mayoi-design", 1, None);
    
    if get_result.is_err() {
        panic!{"Failed to initialize"}
    }

    let tasks = get_result.unwrap();
    let redis_task: Vec<ProjectTaskShrink> = tasks
        .into_iter()
        .filter_map(|it| {
            match ProjectTaskShrink::try_from(it) {
                Ok(task_shrink) => {
                    Some(task_shrink)
                }
                Err(_) => {
                    None
                }
            }
        })
        .collect();
        
    for it in redis_task {
        let _ = redis.put_task(it.task_id, it.task_title);
    }
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
