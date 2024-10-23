mod adapter;
mod redis_wrapper;

use std::{env, thread, time::Duration};
use adapter::{ProjectTaskShrink, create_post_from_task};
use chrono::{Local, Timelike};
use redis_wrapper::RedisWrapper;
use github::client::Client;
use github::model::ProjectTask;
use slack::webhook::Webhook;
use slack::post::Post;

fn main() {
    // This app panics when required variables are not found or failed to connect to redis server

    let slack_end_point = env::var("MAIGO_SLACK_URL").unwrap();
    let github_token = env::var("MAIGO_GITHUB_TOKEN").unwrap();
    let mut redis_client = RedisWrapper::new("redis://redis").unwrap();
    let slack = Webhook::new(slack_end_point.clone());
    let gc = Client::new(&github_token);

    println!{"Main: Success to get env.\n      {}\n      {}", slack_end_point, github_token};

    initialize(&mut redis_client, &gc);

    post_txt_to_channel(
        &slack,
        "Projects Notifier is successfully booted!"
    );

    loop {
        let current_minute = Local::now().time().minute();
        println!{"Main: Current_minute: {}", current_minute};
        if current_minute % 5 == 0 {
            println!{"Main: Trying to fetch Tasks..."};
            let tasks_result = gc.get_project_tasks("mayoi-design", 1, None);
            if tasks_result.is_err() {
                println!{"Main: Fetch error!"};
                post_txt_to_channel(
                    &slack,
                    "Failed to fetch tasks!"
                );
                continue;
            }
            
            let tasks = tasks_result.unwrap();
            let tasks_for_notify: Vec<ProjectTask> = tasks
                .into_iter()
                .filter_map(|it| {
                    let shrink = ProjectTaskShrink::try_from(it.clone()).unwrap();
                    println!{
                        "Main: Looking up the task {}({}) from chache...",
                        it.task_title,
                        it.task_id,
                    };
                    let chache = redis_client.get_task(shrink.task_id.clone());
                    if chache.is_err() {
                        println!{"Failed to fetch chache from Redis!: id={}, title={}", shrink.task_id, shrink.task_title};
                        return None;
                    }

                    println!{"Main: Task {}({}) found in chache!", it.task_title, it.task_id};
                    
                    match chache.unwrap() {
                        Some(chached_title) => {
                            if it.task_title != chached_title {
                                println!{
                                    "Main: Chach found for Task {}({}) but the Title is not same. Updated title is {}",
                                    it.task_title,
                                    it.task_id,
                                    chached_title
                                };
                                Some(it)
                            } else {
                                None
                            }
                        }
                        None => {
                            println!{
                                "Main: Task {}({}) not found in chach. Putting it to redis...",
                                it.task_title,
                                it.task_id
                            };
                            let put_result = redis_client.put_task(shrink.task_id, shrink.task_title);

                            match put_result {
                                Ok(_) => {
                                    println!{
                                        "Main: Task resistered successfully (*^ーﾟ)b"
                                    };
                                    Some(it)
                                }
                                Err(_) => {
                                    println!{
                                        "Main: Put the task {}({}) for redis failed!",
                                        it.task_title,
                                        it.task_id
                                    }
                                     post_txt_to_channel(
                                        &slack,
                                        "Warning! Putting new task to redis failed! This can be an unrecoverable error!\nPlace: Main -> Fetch Task -> New Task -> Resister New Task To Chache"
                                    );
                                    None
                                }
                            }
                        }
                    }
                })
                .collect();

            let posts: Vec<Post<'_>> = tasks_for_notify
                .into_iter()
                .map(|it| create_post_from_task(it))
                .collect();

            for it in posts {
                post_task_to_slack(&slack, it);
            }
            thread::sleep(Duration::from_secs(30));
        }
        thread::sleep(Duration::from_secs(30));
    }
}

fn initialize(
    redis: &mut RedisWrapper,
    gc: &Client
) {
    println!{"Initialize: Start"};
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
    println!{"Initialize: End"};
}

fn post_task_to_slack<'a>(client: &Webhook, post: Post<'a>) {
    match client.post(post) {
        Ok(_) => {
            println!{"Post Success!"}
        }
        Err(_) => {
            println!{"Post Failed"}
        }
    }
}

fn post_txt_to_channel(client: &Webhook, txt: &str) {
    let mut post = Post::new();
    post.add_text_block(txt);

    match client.post(post) {
        Ok(_) => {
            println!{"Main: The txt ({}) successfully posted to the channel", txt}
        }
        Err(_) => {
            println!{"Main: Posting txt to the channel failed"}
        }
    }
}
