use serde::Serialize;

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