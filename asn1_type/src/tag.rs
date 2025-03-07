#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tag(pub u32);

impl From<u32> for Tag {
    fn from(v: u32) -> Self {
        Tag(v)
    }
}

pub trait Tagged {
    const TAG: Tag;
}

impl<T> Tagged for &'_ T
where
    T: Tagged,
{
    const TAG: Tag = T::TAG;
}
