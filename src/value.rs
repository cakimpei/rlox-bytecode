use std::fmt::Display;

#[derive(Clone, Debug)]
pub(crate) struct Val {
    val: f64,
}

impl Val {
    pub(crate) fn new(val: f64) -> Self {
        Val { val }
    }

    pub(crate) fn val(&self) -> &f64 {
        &self.val
    }

    pub(crate) fn val_mut(&mut self) -> &mut f64 {
        &mut self.val
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
