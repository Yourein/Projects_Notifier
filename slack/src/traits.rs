use crate::blocks::{TextBlock, SectionBlock};

#[derive(Debug)]
pub enum Block<'a> {
    TextBlock(TextBlock<'a>),
    SectionBlock(SectionBlock<'a>),
}