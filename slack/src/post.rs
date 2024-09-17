use crate::{blocks::{Attachments, SectionBlock, TextBlock}, traits::*};

pub struct Post<'a> {
    blocks: Vec<Block<'a>>,
    attachments: Vec<Attachments<'a>>
}

impl<'a> Post<'a> {
    pub fn new() -> Post<'a> {
        Post {
            blocks: vec!(),
            attachments: vec!()
        }
    }

    pub fn to_json(&mut self) -> String {
        // TODO: replace this serialize operation with #[derive(Serialize)]
        let blocks_json: String = self.blocks
            .iter()
            .filter_map(|x| {
                match x {
                    Block::TextBlock(textblock) => {
                        serde_json::to_string(textblock).ok()
                    },
                    Block::SectionBlock(sectionblock) => {
                        serde_json::to_string(sectionblock).ok()
                    }
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
            Block::TextBlock(
                TextBlock::new(str)
            )
        );
    }

    pub fn add_section_block(&mut self, block: SectionBlock<'a>) {
        self.blocks.push(
            Block::SectionBlock(
                block
            )
        );
    }

    pub fn add_attachment(&mut self, attachment: Attachments<'a>) {
        self.attachments.push(attachment);
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