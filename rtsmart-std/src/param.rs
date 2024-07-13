use alloc::vec::Vec;
use core::iter::IntoIterator;
use core::ops::Deref;
use core::fmt::{self, Display};

#[derive(Debug)]
pub struct ParamItem(&'static [u8]);

impl Deref for ParamItem {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        return self.0;
    }
}

impl Display for ParamItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", core::str::from_utf8(self.0).unwrap())
    }
}

impl ParamItem {
    pub fn new(raw: &'static [u8]) -> Self {
        ParamItem(raw)
    }
}

pub type Param = <Vec<ParamItem> as IntoIterator>::IntoIter;
