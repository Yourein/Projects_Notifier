use crate::blocks::TextBlock;

pub(crate) enum Blocks<'a> {
    TextBlock(TextBlock<'a>)
}


pub trait Section {}