use crate::blocks::{Text, TextBlock};

pub(crate) enum Blocks<'a> {
    TextBlock(TextBlock<'a>),
    SectionBlock(Text<'a>),
}


pub trait Section {}