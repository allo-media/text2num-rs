//! DigitBuilder
//!
//! Build numeric (base 10) representation using only elementary operations ensuring a
//! valid construction at every step.
//!
//! Everywhere, the term `position` refers to decimal positions: 0 is units, 1 is tens, etc…

use std::ops::Deref;

use super::error::Error;
use super::lang::MorphologicalMarker;

#[derive(Debug)]
pub struct DigitString {
    buffer: Vec<u8>,
    leading_zeroes: usize,
    frozen: bool,
    /// Decoder state if any
    pub flags: u64,
    pub marker: MorphologicalMarker,
}

fn all_zeros(slice: &[u8]) -> bool {
    slice.iter().all(|&c| c == b'0')
}

impl DigitString {
    pub fn new() -> DigitString {
        DigitString {
            buffer: Vec::with_capacity(4),
            leading_zeroes: 0,
            frozen: false,
            flags: 0,
            marker: MorphologicalMarker::None,
        }
    }

    /// Clear DigitString as if it was brand new.
    pub fn reset(&mut self) {
        self.leading_zeroes = 0;
        self.frozen = false;
        self.marker = MorphologicalMarker::None;
        self.buffer.clear();
    }

    /// Freeze the DigitSring to signal the number is complete.
    ///
    /// Useful for languages that use some kind of flexion or suffix to mark the end.
    /// (for example, the suffix -th in English ordinals).
    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    /// Put the given digit string in the buffer, right aligned.
    ///
    /// Return an error if slots are not free or not 0 or digit string is frozen.
    /// Special case for `0`:
    /// * only valid in leading position (that is, the buffer still evaluates to 0)
    /// * any number of leading zeroes are accepted and kept.
    pub fn put(&mut self, digits: &[u8]) -> Result<(), Error> {
        if self.frozen {
            return Err(Error::Frozen);
        }
        if self.buffer.is_empty() && digits == b"0" {
            self.leading_zeroes += 1;
            return Ok(());
        }
        if all_zeros(digits) {
            return Err(Error::Overlap);
        }
        let positions = digits.len();
        match self.buffer.len() {
            0 => {
                self.buffer.extend_from_slice(digits);
                Ok(())
            }
            l if l < positions => Err(Error::Overlap),
            l if all_zeros(&self.buffer[(l - positions)..]) => {
                self.buffer[(l - positions)..].copy_from_slice(digits);
                Ok(())
            }
            _ => Err(Error::Overlap),
        }
    }

    /// put a single non nul digit at a given position > 0 from the right. The position must be free (empty or 0)
    ///
    /// If new positions are created in between, they are filled with zeros.
    pub fn put_digit_at(&mut self, digit: u8, position: usize) -> Result<(), Error> {
        if self.frozen {
            return Err(Error::Frozen);
        }
        if digit == b'0' {
            return Err(Error::Overlap);
        }
        let len = self.buffer.len();
        if position >= len {
            let mut new_buffer = Vec::with_capacity(position + 3);
            new_buffer.resize(position + 1, b'0');
            new_buffer[0] = digit;
            new_buffer[position + 1 - len..].copy_from_slice(&self.buffer);
            self.buffer = new_buffer;
            Ok(())
        } else if self.buffer[len - 1 - position] == b'0' {
            self.buffer[len - 1 - position] = digit;
            Ok(())
        } else {
            Err(Error::Overlap)
        }
    }

    /// push the given digit string at the right, appending it to the digits already in the buffer.
    pub fn push(&mut self, digits: &[u8]) -> Result<(), Error> {
        self.buffer.extend_from_slice(digits);
        Ok(())
    }

    /// Force put (never fail, unless `self` is frozen)
    pub fn fput(&mut self, digits: &[u8]) -> Result<(), Error> {
        if self.frozen {
            return Err(Error::Frozen);
        }
        let positions = digits.len();
        match self.buffer.len() {
            0 => {
                self.buffer.extend_from_slice(digits);
                Ok(())
            }
            mut l => {
                if l < positions {
                    self.buffer.resize(positions, b'0');
                    l = positions;
                }
                self.buffer[(l - positions)..].copy_from_slice(digits);
                Ok(())
            }
        }
    }

    /// Peek the `positions` right most digits.
    pub fn peek(&self, positions: usize) -> &[u8] {
        let length = self.buffer.len();
        let range = length.min(positions);
        &self.buffer[(length - range)..]
    }

    /// Return true if the rightmorst `positions` positions are all free (empty or 0).
    pub fn is_free(&self, positions: usize) -> bool {
        self.is_empty() || self.peek(positions).iter().all(|&c| c == b'0')
    }

    /// Range is inclusive on both ends.
    pub fn is_range_free(&self, start_position: usize, end_position: usize) -> bool {
        debug_assert!(start_position < end_position);
        if start_position >= self.buffer.len() {
            return true;
        }
        let left_bound = if end_position >= self.buffer.len() {
            0
        } else {
            self.buffer.len() - end_position - 1
        };
        all_zeros(&self.buffer[left_bound..self.buffer.len() - start_position])
    }

    pub fn is_position_free(&self, position: usize) -> bool {
        let max_pos = self.buffer.len() - 1;
        position > max_pos || self.buffer[max_pos - position] == b'0'
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty() && self.leading_zeroes == 0
    }

    pub fn len(&self) -> usize {
        self.buffer.len() + self.leading_zeroes
    }

    /// Shift the `positions` right most digits, `positions` slots to the left.
    ///
    /// Return an error if destination slots are  not free or not 0 or string is frozen.
    /// If there is  nothing on the starting position, first puts 1.
    pub fn shift(&mut self, positions: usize) -> Result<(), Error> {
        if self.frozen {
            return Err(Error::Frozen);
        }
        if positions == 0 {
            return Ok(());
        }
        if self.buffer.is_empty() {
            self.buffer.push(b'1');
        }
        let l = self.buffer.len();
        if l <= positions {
            return {
                self.buffer.resize(l + positions, b'0');
                Ok(())
            };
        }
        let mut padding_zeroes = self.buffer[(l - positions)..]
            .iter()
            .take_while(|&c| *c == b'0')
            .count();
        if padding_zeroes == positions {
            self.buffer[l - 1] = b'1';
            padding_zeroes -= 1;
        }
        let span = 2 * positions - padding_zeroes;
        if l >= span && all_zeros(&self.buffer[(l - span)..(l - positions)]) {
            let (left, right) = self.buffer.split_at_mut(l - positions);
            left[(l - span)..].swap_with_slice(&mut right[padding_zeroes..]);
            Ok(())
        } else {
            Err(Error::Overlap)
        }
    }

    /// Formal base 10 string representation with leading zeroes
    pub fn to_string(&self) -> String {
        // we know that the string is valid.
        let mut res = "0".repeat(self.leading_zeroes);
        res.push_str(std::str::from_utf8(self.buffer.as_slice()).unwrap());
        res
    }

    pub fn is_ordinal(&self) -> bool {
        self.marker.is_ordinal()
    }
}

impl Deref for DigitString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl Default for DigitString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_put_single() -> Result<(), Error> {
        let mut builder = DigitString::new();
        builder.put(b"5")
    }

    #[test]
    fn test_put_twice_ok() -> Result<(), Error> {
        let mut builder = DigitString::new();
        builder.put(b"50")?;
        builder.put(b"5")
    }

    #[test]
    fn test_put_twice_ok2() -> Result<(), Error> {
        let mut builder = DigitString::new();
        builder.put(b"500")?;
        builder.put(b"55")
    }

    #[test]
    fn test_put_twice_overlap() {
        let mut builder = DigitString::new();
        builder.put(b"5").expect("should pass");
        assert!(builder.put(b"22").is_err());
        assert!(builder.put(b"2").is_err());
    }

    #[test]
    fn test_put_twice_no_slot() {
        let mut builder = DigitString::new();
        builder.put(b"52").expect("should pass");
        assert!(builder.put(b"2").is_err());
    }

    #[test]
    fn test_zero() {
        let mut builder = DigitString::new();
        assert!(builder.put(b"0").is_ok());
        assert!(builder.put(b"0").is_ok());
        assert!(builder.put(b"5").is_ok());
    }

    #[test]
    fn test_zeroes() {
        let mut builder = DigitString::new();
        assert!(builder.put(b"00").is_err());
        assert!(builder.put(b"000").is_err());
    }

    #[test]
    fn test_peek1() {
        let mut builder = DigitString::new();
        builder.put(b"5").expect("should pass");
        assert_eq!(builder.peek(1), b"5");
        assert_eq!(builder.peek(2), b"5");
        assert_eq!(builder.peek(3), b"5");
    }

    #[test]
    fn test_peek2() {
        let mut builder = DigitString::new();
        builder.put(b"56").expect("should pass");
        assert_eq!(builder.peek(1), b"6");
        assert_eq!(builder.peek(2), b"56");
        assert_eq!(builder.peek(3), b"56");
    }

    #[test]
    fn test_fput() -> Result<(), Error> {
        let mut builder = DigitString::new();
        builder.fput(b"5")?;
        builder.fput(b"8")?;
        builder.fput(b"73")?;
        builder.fput(b"5")
    }

    #[test]
    fn test_shift_single() -> Result<(), Error> {
        let mut builder = DigitString::new();
        builder.fput(b"5")?;
        builder.shift(3)?;
        assert_eq!(builder.peek(4), b"5000");
        Ok(())
    }

    #[test]
    fn test_shift_shorter() -> Result<(), Error> {
        let mut builder = DigitString::new();
        builder.fput(b"51")?;
        builder.shift(2)?;
        assert_eq!(builder.peek(4), b"5100");
        Ok(())
    }

    #[test]
    fn test_shift_subslice_ok() -> Result<(), Error> {
        let mut builder = DigitString::new();
        builder.fput(b"50032")?;
        builder.shift(2)?;
        assert_eq!(builder.peek(6), b"53200");
        Ok(())
    }

    #[test]
    fn test_shift_subslice_ok2() -> Result<(), Error> {
        let mut builder = DigitString::new();
        builder.fput(b"2007")?;
        builder.shift(2)?;
        assert_eq!(builder.peek(6), b"2700");
        Ok(())
    }

    #[test]
    fn test_shift_subslice_overlap() {
        let mut builder = DigitString::new();
        builder.fput(b"51032").expect("should pass");
        assert!(builder.shift(2).is_err());
    }

    #[test]
    fn test_shift_subslice_overlap_short() {
        let mut builder = DigitString::new();
        builder.fput(b"532").expect("should pass");
        assert!(builder.shift(2).is_err());
    }

    #[test]
    fn test_shift_empty() {
        let mut builder = DigitString::new();
        builder.shift(2).unwrap();
        assert_eq!(builder.to_string(), "100")
    }

    #[test]
    fn test_shift_full_zeroes() {
        let mut builder = DigitString::new();
        builder.put(b"1000").unwrap();
        builder.shift(2).unwrap();
        assert_eq!(builder.to_string(), "1100")
    }

    #[test]
    fn complete_example() -> Result<(), Error> {
        // 2792
        let mut builder = DigitString::new();
        builder.put(b"2")?;
        builder.shift(3)?;
        builder.put(b"7")?;
        builder.shift(2)?;
        builder.put(b"90")?;
        builder.put(b"2")?;
        assert_eq!(builder.peek(5), b"2792");
        Ok(())
    }

    #[test]
    fn complete_example_leading_zeroes() -> Result<(), Error> {
        // 2792
        let mut builder = DigitString::new();
        builder.put(b"0")?;
        builder.put(b"0")?;
        builder.put(b"2")?;
        builder.shift(3)?;
        builder.put(b"7")?;
        builder.shift(2)?;
        builder.put(b"90")?;
        builder.put(b"2")?;
        assert_eq!(builder.to_string(), "002792");
        Ok(())
    }

    #[test]
    fn test_put_digit_at() {
        let mut builder = DigitString::new();
        builder.put_digit_at(b'1', 3).unwrap();
        assert_eq!(builder.to_string(), "1000");
        assert!(builder.put_digit_at(b'0', 2).is_err());
        builder.put_digit_at(b'2', 1).unwrap();
        assert!(builder.put_digit_at(b'3', 1).is_err());
        assert_eq!(builder.to_string(), "1020");
    }

    #[test]
    fn test_is_range_free() {
        let mut dstring = DigitString::new();
        dstring.buffer = Vec::from(b"200000");
        assert!(dstring.is_range_free(6, 12));
        assert!(dstring.is_range_free(3, 4));
        assert!(!dstring.is_range_free(3, 5));
        assert!(!dstring.is_range_free(3, 10));
    }

    #[test]
    fn test_is_position_free() {
        let mut dstring = DigitString::new();
        dstring.buffer = Vec::from(b"203070");
        assert!(dstring.is_position_free(0));
        assert!(dstring.is_position_free(2));
        assert!(dstring.is_position_free(4));
        assert!(!dstring.is_position_free(1));
        assert!(!dstring.is_position_free(3));
        assert!(!dstring.is_position_free(5));
    }
}
