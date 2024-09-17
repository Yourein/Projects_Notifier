use crate::blocks::{TextBlock, SectionBlock};

#[derive(Debug)]
pub(crate) enum Block<'a> {
    TextBlock(TextBlock<'a>),
    SectionBlock(SectionBlock<'a>),
}