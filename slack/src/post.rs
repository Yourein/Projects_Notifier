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
        let blocks_json: String = self.blocks
            .iter()
            .filter_map(|x| {
                match x {
                    Blocks::TextBlock(textblock) => {
                        serde_json::to_string(textblock).ok()
                    },
                }
            })
            .collect::<Vec<String>>()
            .join(",");

        //TODO: Section

        format!{"{{blocks: [{}]}}", blocks_json}
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

#[cfg(test)]
mod tests {
    use super::Post;

    #[test]
    fn json() {
        let mut post = Post::new();
        post.add_text_block("hoge");

        let json = post.to_json();
        eprintln!{"{}", json}
        assert_eq!(1, 1);
    }
}