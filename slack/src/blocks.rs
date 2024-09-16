use serde::Serialize;
use serde::ser::{SerializeSeq, Serializer};
use crate::traits::Block;

#[derive(Debug)]
pub(crate) struct Blocks<'a>(pub Vec<Block<'a>>);

impl<'a> Serialize for Blocks<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for e in &self.0 {
            match e {
                Block::SectionBlock(e) => {
                    seq.serialize_element(e)?;
                }
                Block::TextBlock(e) => {
                    seq.serialize_element(e)?;
                }
            }
        }
        seq.end()
    }
}

#[derive(Serialize, Debug)]
struct Text<'a> {
    #[serde(rename = "type")]
    _type: &'a str,
    text: String,
}

#[derive(Serialize, Debug)]
pub(crate) struct TextBlock<'a> {
    #[serde(rename = "type")]
    _type: &'a str,
    text: Text<'a>,
}

impl<'a> TextBlock<'_> {
    pub fn new(text: String) -> TextBlock<'a> {
        TextBlock {
            _type: "section",
            text: Text {
                _type: "mrkdwn",
                text: text,
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct SectionBlock<'a> {
    #[serde(rename = "type")]
    _type: &'a str,
    fields: Vec<Text<'a>>
}

impl<'a> SectionBlock<'_> {
    pub fn new() -> Self {
        Self {
            _type: "section",
            fields: vec!(),
        }
    }

    pub fn add_paragraph(&mut self, title: &str, body: &str) {
        let text = format!{"*{}*\n{}", title, body};

        self.fields.push(
            Text {
                _type: "section",
                text: text,
            }
        )
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct Attachments<'a> {
    color: String,
    blocks: Blocks<'a>,
}

impl<'a> Attachments<'a> {
    pub fn new(color: Option<String>) -> Self {
        Self {
            color: color.unwrap_or_else(|| "#f2c744".to_string()),
            blocks: Blocks(vec!()),
        }
    }

    pub fn add_block(&mut self, block: Block<'a>) {
        self.blocks.0.push(block);
    }
}

#[cfg(test)]
mod tests {
    use crate::{blocks::{Attachments, SectionBlock, TextBlock}, traits::Block};

    #[test]
    fn Serialize_Attachments() {
        let mut attachments = Attachments::new(None);
        attachments.add_block(
            Block::TextBlock(
                TextBlock::new("hoge".to_string())
            )
        );

        attachments.add_block(
            Block::SectionBlock(
                SectionBlock::new()
            )
        );

        let res = serde_json::to_string(&attachments).unwrap();
        eprintln!{"{}", res};
        assert!(true)
    }
}