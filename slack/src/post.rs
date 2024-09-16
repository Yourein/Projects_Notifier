use crate::{blocks::TextBlock, traits::*};

pub struct Post {
    blocks: Vec<Box<dyn Block>>,
    sections: Vec<Box<dyn Section>>
}

impl Post {
    pub fn new() -> Post {
        Post {
            blocks: vec!(),
            sections: vec!()
        }
    }

    pub fn to_json(&mut self) -> String {
        todo!()
    }

    pub fn add_text_block(&mut self, text: &str) {
        let str = text.to_string();
        self.blocks.push(TextBlock::new(str));
    }
}