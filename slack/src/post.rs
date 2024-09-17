use crate::{blocks::{Attachments, Blocks, SectionBlock, TextBlock}, traits::*};
use serde::Serialize;

#[derive(Serialize)]
pub struct Post<'a> {
    blocks: Blocks<'a>,
    attachments: Vec<Attachments<'a>>
}

impl<'a> Post<'a> {
    pub fn new() -> Post<'a> {
        Post {
            blocks: Blocks(vec!()),
            attachments: vec!()
        }
    }

    pub fn add_text_block(&mut self, text: &str) {
        let str = text.to_string();
        self.blocks.0.push(
            Block::TextBlock(
                TextBlock::new(str)
            )
        );
    }

    pub fn add_section_block(&mut self, block: SectionBlock<'a>) {
        self.blocks.0.push(
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
    use crate::blocks::{Attachments, SectionBlock, TextBlock};
    use crate::traits::Block;

    use super::Post;

    #[test]
    fn mock_json() {
        let mut post = Post::new();
        post.add_text_block("hoge");

        let mut attachment = Attachments::new(None);
        let mut section = SectionBlock::new();
        section.add_paragraph("Test Title", "test body");
        attachment.add_block(
            Block::TextBlock(TextBlock::new("attachment text".to_string()))
        );
        attachment.add_block(
            Block::SectionBlock(section)
        );
        post.add_attachment(attachment);

        let serde_json = serde_json::to_string(&post).unwrap();
        eprintln!{"{}", serde_json}
        assert!(true)
    }
}