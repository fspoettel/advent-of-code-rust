use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

/* -------------------------------------------------------------------------- */

/// A valid day number of advent (i.e. an integer in range 1 to 25).
///
/// # Display
/// This value is display as a two digit number.
///
/// ```
/// # use advent_of_code::Day;
/// let day = Day::new(8).unwrap();
/// assert_eq!(day.to_string(), "08")
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Day(u8);

impl Day {
    /// Creates a [`Day`] from the provided value if it's in the valid range,
    /// returns [`None`] otherwise.
    pub fn new(day: u8) -> Option<Self> {
        if day == 0 || day > 25 {
            return None;
        }
        Some(Self(day))
    }

    fn new_unchecked(day: u8) -> Self {
        debug_assert!(day != 0 && day <= 25);
        Self(day)
    }

    /// Converts the [`Day`] into an [`u8`].
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl PartialEq<u8> for Day {
    fn eq(&self, other: &u8) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<u8> for Day {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

/* -------------------------------------------------------------------------- */

impl FromStr for Day {
    type Err = DayFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day = s.parse().map_err(|_| DayFromStrError)?;
        Self::new(day).ok_or(DayFromStrError)
    }
}

/// An error which can be returned when parsing a [`Day`].
#[derive(Debug)]
pub struct DayFromStrError;

impl Error for DayFromStrError {}

impl Display for DayFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("expecting a day number between 1 and 25")
    }
}

/* -------------------------------------------------------------------------- */

/// An iterator that yield every day of advent from the 1st to the 25th.
pub fn every_day() -> EveryDay {
    EveryDay { current: 1 }
}

/// An iterator that yield every day of advent from the 1st to the 25th.
pub struct EveryDay {
    current: u8,
}

impl Iterator for EveryDay {
    type Item = Day;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 25 {
            return None;
        }
        // NOTE: the iterator start at 1 and we have verified that the value is not above 25.
        let day = Day::new_unchecked(self.current);
        self.current += 1;

        Some(day)
    }
}

/* -------------------------------------------------------------------------- */

#[cfg(feature = "test_lib")]
mod tests {
    use super::{every_day, Day};

    #[test]
    fn every_day_iterator() {
        let mut iter = every_day();

        assert_eq!(iter.next(), Some(Day::new_unchecked(1)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(2)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(3)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(4)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(5)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(6)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(7)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(8)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(9)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(10)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(11)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(12)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(13)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(14)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(15)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(16)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(17)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(18)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(19)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(20)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(21)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(22)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(23)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(24)));
        assert_eq!(iter.next(), Some(Day::new_unchecked(25)));
        assert_eq!(iter.next(), None);
    }
}

/* -------------------------------------------------------------------------- */
