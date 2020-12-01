//! DigitBuilder
//!
//! Build numeric representation using only elementary operations ensuring a
//! valid construction at every step.

use super::error::Error;

#[derive(Debug)]
pub struct DigitString {
    buffer: Vec<u8>,
    leading_zeroes: usize,
    frozen: bool,
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
        }
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
            0 => Ok(self.buffer.extend_from_slice(digits)),
            l if l < positions => Err(Error::Overlap),
            l if all_zeros(&self.buffer[(l - positions)..]) => {
                self.buffer[(l - positions)..].copy_from_slice(digits);
                Ok(())
            }
            _ => Err(Error::Overlap),
        }
    }

    /// Force put (never fail, unless dstring is frozen)
    pub fn fput(&mut self, digits: &[u8]) -> Result<(), Error> {
        if self.frozen {
            return Err(Error::Frozen);
        }
        let positions = digits.len();
        match self.buffer.len() {
            0 => Ok(self.buffer.extend_from_slice(digits)),
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

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
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
            return Ok(self.buffer.resize(l + positions, b'0'));
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

    /// Formal decimal string reprensentation with leading zeroes
    pub fn into_string(self) -> String {
        // we know that the string is valid.
        let mut res = "0".repeat(self.leading_zeroes);
        res.push_str(&String::from_utf8(self.buffer).unwrap());
        res
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
        assert_eq!(builder.into_string(), "100")
    }

    #[test]
    fn test_shift_full_zeroes() {
        let mut builder = DigitString::new();
        builder.put(b"1000").unwrap();
        builder.shift(2).unwrap();
        assert_eq!(builder.into_string(), "1100")
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
        assert_eq!(builder.into_string(), "002792");
        Ok(())
    }
}
