#[derive(Debug)]
pub enum Error {
    Overlap,
    NaN,
    Incomplete,
    Frozen,
}
