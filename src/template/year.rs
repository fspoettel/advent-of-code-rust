use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

extern crate chrono;
use chrono::{Datelike, FixedOffset, Utc};

const SERVER_UTC_OFFSET: i32 = -5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Year(i32);

impl Year {
    /// Creates a [`Year`] from the provided value if it's in the valid range,
    /// returns [`None`] otherwise.
    pub fn new(year: i32) -> Option<Self> {
        if 2015 <= year && year <= Year::last_year().into_inner() {
            Some(Self(year))
        } else {
            None
        }
    }

    // Not part of the public API
    #[doc(hidden)]
    pub const fn __new_unchecked(year: i32) -> Self {
        Self(year)
    }

    /// Converts the [`year`] into an [`i32`].
    pub fn into_inner(self) -> i32 {
        self.0
    }

    pub fn last_year() -> Self {
        let offset = FixedOffset::east_opt(SERVER_UTC_OFFSET * 3600).unwrap();
        let today = Utc::now().with_timezone(&offset);
        if today.month() == 12 {
            Self::__new_unchecked(today.year())
        } else {
            // December is not here yet, so last AoC was last year
            Self::__new_unchecked(today.year() - 1)
        }
    }

    /// Returns the current year.
    pub fn this_year() -> Option<Self> {
        let offset = FixedOffset::east_opt(SERVER_UTC_OFFSET * 3600)?;
        let today = Utc::now().with_timezone(&offset);
        Self::new(today.year())
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

impl PartialEq<i32> for Year {
    fn eq(&self, other: &i32) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<i32> for Year {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

/* -------------------------------------------------------------------------- */

impl FromStr for Year {
    type Err = YearFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year = s.parse().map_err(|_| YearFromStrError)?;
        Self::new(year).ok_or(YearFromStrError)
    }
}

/// An error which can be returned when parsing a [`year`].
#[derive(Debug)]
pub struct YearFromStrError;

impl Error for YearFromStrError {}

impl Display for YearFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "expecting a year number between 2015 and {}",
                Year::last_year()
            )
            .as_str(),
        )
    }
}
