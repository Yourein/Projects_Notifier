use serde::Serialize;
use crate::traits::Blocks;

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