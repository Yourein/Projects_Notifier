use crate::{blocks::TextBlock, traits::*};

pub struct Post<'a> {
    blocks: Vec<Blocks<'a>>,
    sections: Vec<Box<dyn Section>>
}

impl<'a> Post<'_> {
    pub fn new() -> Post<'a> {
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
        self.blocks.push(
            Blocks::TextBlock(
                TextBlock::new(str)
            )
        );
    }
}