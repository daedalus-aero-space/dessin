use algebr::Vec2;

use crate::style::Style;

pub type Line = LineBuilder<true>;

#[derive(Debug, Clone)]
pub struct LineBuilder<const IS_INIT: bool> {
    pub(crate) from: Vec2,
    pub(crate) to: Vec2,
    pub(crate) style: Option<Style>,
}
crate::impl_style!(LineBuilder<true>);
impl<const IS_INIT: bool> LineBuilder<IS_INIT> {
    pub const fn from(from: Vec2) -> LineBuilder<false> {
        LineBuilder {
            from,
            to: from,
            style: None,
        }
    }
}

impl LineBuilder<false> {
    pub const fn to(self, to: Vec2) -> LineBuilder<true> {
        LineBuilder {
            from: self.from,
            to,
            style: self.style,
        }
    }
}
