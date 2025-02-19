use super::tag::Tag;

pub trait Choice {
    fn tag() -> Tag;
}
