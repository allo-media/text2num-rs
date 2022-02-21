#[derive(Debug)]
/// Number recognition errors.
/// They are mostly used internally by the high level API.
pub enum Error {
    /// The latest data overlaps with the number currently decoded.
    /// It's likely the start of a new number.
    Overlap,
    /// That's not a number!
    NaN,
    /// The number being decoded is incomplete and we need more data
    Incomplete,
    /// The currently decoded number is complete and we don't accept new data
    Frozen
}
