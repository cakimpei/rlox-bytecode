use std::fmt::Display;

pub(crate) struct Val {
    val: f32,
}

impl Val {
    pub(crate) fn new(val: f32) -> Self {
        Val { val }
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
