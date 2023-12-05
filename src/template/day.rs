use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

/// A valid day number of advent (i.e. an integer in range 1 to 25).
///
/// # Display
/// This value displays as a two digit number.
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

    // Not part of the public API
    #[doc(hidden)]
    pub const fn __new_unchecked(day: u8) -> Self {
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

/// An iterator that yields every day of advent from the 1st to the 25th.
pub fn all_days() -> AllDays {
    AllDays::new()
}

/// An iterator that yields every day of advent from the 1st to the 25th.
pub struct AllDays {
    current: u8,
}

impl AllDays {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { current: 1 }
    }
}

impl Iterator for AllDays {
    type Item = Day;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 25 {
            return None;
        }
        // NOTE: the iterator starts at 1 and we have verified that the value is not above 25.
        let day = Day(self.current);
        self.current += 1;

        Some(day)
    }
}

/* -------------------------------------------------------------------------- */

/// Creates a [`Day`] value in a const context.
#[macro_export]
macro_rules! day {
    ($day:expr) => {{
        const _ASSERT: () = assert!(
            $day != 0 && $day <= 25,
            concat!(
                "invalid day number `",
                $day,
                "`, expecting a value between 1 and 25"
            ),
        );
        $crate::template::Day::__new_unchecked($day)
    }};
}

/* -------------------------------------------------------------------------- */

#[cfg(feature = "test_lib")]
mod tests {
    use super::{all_days, Day};

    #[test]
    fn all_days_iterator() {
        let mut iter = all_days();

        assert_eq!(iter.next(), Some(Day(1)));
        assert_eq!(iter.next(), Some(Day(2)));
        assert_eq!(iter.next(), Some(Day(3)));
        assert_eq!(iter.next(), Some(Day(4)));
        assert_eq!(iter.next(), Some(Day(5)));
        assert_eq!(iter.next(), Some(Day(6)));
        assert_eq!(iter.next(), Some(Day(7)));
        assert_eq!(iter.next(), Some(Day(8)));
        assert_eq!(iter.next(), Some(Day(9)));
        assert_eq!(iter.next(), Some(Day(10)));
        assert_eq!(iter.next(), Some(Day(11)));
        assert_eq!(iter.next(), Some(Day(12)));
        assert_eq!(iter.next(), Some(Day(13)));
        assert_eq!(iter.next(), Some(Day(14)));
        assert_eq!(iter.next(), Some(Day(15)));
        assert_eq!(iter.next(), Some(Day(16)));
        assert_eq!(iter.next(), Some(Day(17)));
        assert_eq!(iter.next(), Some(Day(18)));
        assert_eq!(iter.next(), Some(Day(19)));
        assert_eq!(iter.next(), Some(Day(20)));
        assert_eq!(iter.next(), Some(Day(21)));
        assert_eq!(iter.next(), Some(Day(22)));
        assert_eq!(iter.next(), Some(Day(23)));
        assert_eq!(iter.next(), Some(Day(24)));
        assert_eq!(iter.next(), Some(Day(25)));
        assert_eq!(iter.next(), None);
    }
}

/* -------------------------------------------------------------------------- */
