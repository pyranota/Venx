mod get;
mod set;

#[derive(Clone, Copy)]
pub enum EntryOpts {
    All,
    Single(u32),
}

pub type LayerOpts = EntryOpts;
