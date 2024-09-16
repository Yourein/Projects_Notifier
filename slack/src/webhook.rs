use crate::post::Post;

pub struct Webhook {
    end_point: String
}

impl Webhook {
    pub fn new(end_point: String) -> Webhook {
        Webhook {
            end_point: end_point
        }
    }

    pub fn post(post: Post) -> Result<(), String> {
        todo!()
    }
}