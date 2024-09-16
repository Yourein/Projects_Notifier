use crate::blocks::{TextBlock, SectionBlock};

pub(crate) enum Blocks<'a> {
    TextBlock(TextBlock<'a>),
    SectionBlock(SectionBlock<'a>),
}


pub trait Section {}