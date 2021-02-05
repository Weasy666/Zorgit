use std::cmp::max;
use std::fmt;
use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use bytesize::ByteSize;

pub trait RenderTime {
    fn rfc3339(&self) -> String;
}

// impl RenderTime for Duration {
//     fn rfc3339(&self) -> String {
//         self.format("%a, %e %b %Y, %T").to_string()
//     }
// }

impl<TZ> RenderTime for DateTime<TZ>
where
    TZ: TimeZone,
{
    fn rfc3339(&self) -> String {
        self.naive_utc().format("%a, %e %b %Y, %T").to_string()
    }
}

impl RenderTime for NaiveDateTime {
    fn rfc3339(&self) -> String {
        self.format("%a, %e %b %Y, %T").to_string()
    }
}

/// This was taken from the chrono-humanize crate (https://crates.io/crates/chrono-humanize)(https://gitlab.com/imp/chrono-humanize-rs)

/// Present the object in human friendly text form
pub trait Humanize {
    /// Emits `String` that represents current object in human friendly form
    fn humanize(&self) -> String;
}

/// Indicates the time of the period in relation to the time of the utterance
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tense {
    Past,
    Present,
    Future,
}

/// The accuracy of the representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Accuracy {
    /// Rough approximation, easy to grasp, but not necessarily accurate
    Rough,
    /// Concise expression, accurate, but not necessarily easy to grasp
    Precise,
}

impl Accuracy {
    /// Returns whether this accuracy is precise
    pub fn is_precise(self) -> bool {
        self == Accuracy::Precise
    }

    /// Returns whether this accuracy is rough
    pub fn is_rough(self) -> bool {
        self == Accuracy::Rough
    }
}

impl From<bool> for Accuracy {
    fn from(precise: bool) -> Self {
        if precise {
            Accuracy::Precise
        } else {
            Accuracy::Rough
        }
    }
}

// Number of seconds in various time periods
const MINUTE: i64 = 60;
const HOUR: i64 = MINUTE * 60;
const DAY: i64 = HOUR * 24;
const WEEK: i64 = DAY * 7;
const MONTH: i64 = DAY * 30;
const YEAR: i64 = DAY * 365;

#[derive(Debug)]
enum TimePeriod {
    Now,
    Seconds(i64),
    Minutes(i64),
    Hours(i64),
    Days(i64),
    Weeks(i64),
    Months(i64),
    Years(i64),
    Eternity,
}

impl TimePeriod {
    fn to_string_precise(&self) -> String {
        use self::TimePeriod::*;
        match *self {
            Now => String::from("now"),
            Seconds(1) => String::from("1 second"),
            Seconds(n) => format!("{} seconds", n),
            Minutes(1) => String::from("1 minute"),
            Minutes(n) => format!("{} minutes", n),
            Hours(1) => String::from("1 hour"),
            Hours(n) => format!("{} hours", n),
            Days(1) => String::from("1 day"),
            Days(n) => format!("{} days", n),
            Weeks(1) => String::from("1 week"),
            Weeks(n) => format!("{} weeks", n),
            Months(1) => String::from("1 month"),
            Months(n) => format!("{} months", n),
            Years(1) => String::from("1 year"),
            Years(n) => format!("{} years", n),
            Eternity => String::from("eternity"),
        }
    }

    fn to_string_rough(&self) -> String {
        use self::TimePeriod::*;
        match *self {
            Now => String::from("now"),
            Seconds(1) => String::from("a second"),
            Seconds(n) => format!("{} seconds", n),
            Minutes(1) => String::from("a minute"),
            Minutes(n) => format!("{} minutes", n),
            Hours(1) => String::from("an hour"),
            Hours(n) => format!("{} hours", n),
            Days(1) => String::from("a day"),
            Days(n) => format!("{} days", n),
            Weeks(1) => String::from("a week"),
            Weeks(n) => format!("{} weeks", n),
            Months(1) => String::from("a month"),
            Months(n) => format!("{} months", n),
            Years(1) => String::from("a year"),
            Years(n) => format!("{} years", n),
            Eternity => String::from("eternity"),
        }
    }

    fn to_string(&self, accuracy: Accuracy) -> String {
        match accuracy {
            Accuracy::Rough => self.to_string_rough(),
            Accuracy::Precise => self.to_string_precise(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HumanTime(Duration);

impl HumanTime {
    pub fn to_text_en(&self, accuracy: Accuracy, tense: Tense) -> String {
        let mut periods = match accuracy {
            Accuracy::Rough => self.rough_period(),
            Accuracy::Precise => self.precise_period(),
        };

        let first = periods.remove(0);
        let last = periods.pop();

        let append = |acc: String, p: TimePeriod| format!("{}, {}", acc, p.to_string(accuracy));
        let mut text = periods.into_iter().fold(first.to_string(accuracy), append);

        if let Some(last) = last {
            text = format!("{} and {}", text, last.to_string(accuracy));
        }

        match tense {
            Tense::Past => format!("{} ago", text),
            Tense::Future => format!("in {}", text),
            Tense::Present => text.to_string(),
        }
    }

    fn tense(&self, accuracy: Accuracy) -> Tense {
        match self.0.num_seconds() {
            -10..=10 if accuracy.is_rough() => Tense::Present,
            seconds if seconds.is_negative() => Tense::Past,
            seconds if seconds.is_positive() => Tense::Future,
            _ => Tense::Present,
        }
    }

    fn rough_period(&self) -> Vec<TimePeriod> {
        use self::TimePeriod::*;

        let period = match self.0.num_seconds().abs() {
            n if n > 547 * DAY => Years(max(n / YEAR, 2)),
            n if n > 345 * DAY => Years(1),
            n if n > 45 * DAY => Months(max(n / MONTH, 2)),
            n if n > 29 * DAY => Months(1),
            n if n > 10 * DAY + 12 * HOUR => Weeks(max(n / WEEK, 2)),
            n if n > 6 * DAY + 12 * HOUR => Weeks(1),
            n if n > 36 * HOUR => Days(max(n / DAY, 2)),
            n if n > 22 * HOUR => Days(1),
            n if n > 90 * MINUTE => Hours(max(n / HOUR, 2)),
            n if n > 45 * MINUTE => Hours(1),
            n if n > 90 => Minutes(max(n / MINUTE, 2)),
            n if n > 45 => Minutes(1),
            n if n > 10 => Seconds(n),
            0..=10 => Now,
            _ => Eternity,
        };

        vec![period]
    }

    fn precise_period(&self) -> Vec<TimePeriod> {
        use self::TimePeriod::*;

        let zero = Duration::zero().num_seconds();

        let mut duration = self.0.num_seconds().abs();
        let mut periods = Vec::<TimePeriod>::new();

        if duration >= YEAR {
            periods.push(Years(duration / YEAR));
            duration %= YEAR;
        }

        if duration >= MONTH {
            periods.push(Months(duration / MONTH));
            duration %= MONTH;
        }

        if duration >= WEEK {
            periods.push(Weeks(duration / WEEK));
            duration %= WEEK;
        }

        if duration >= DAY {
            periods.push(Days(duration / DAY));
            duration %= DAY;
        }

        if duration >= HOUR {
            periods.push(Hours(duration / HOUR));
            duration %= HOUR;
        }

        if duration >= MINUTE {
            periods.push(Minutes(duration / MINUTE));
            duration %= MINUTE;
        }

        if duration > zero || periods.is_empty() {
            periods.push(Seconds(duration));
        }

        periods
    }

    fn locale_en(&self, accuracy: Accuracy) -> String {
        let tense = self.tense(accuracy);
        self.to_text_en(accuracy, tense)
    }
}

impl fmt::Display for HumanTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let accuracy = f.alternate().into();
        f.pad(&self.locale_en(accuracy))
    }
}

impl From<Duration> for HumanTime {
    fn from(duration: Duration) -> Self {
        HumanTime(duration)
    }
}

impl<TZ> From<DateTime<TZ>> for HumanTime
where
    TZ: TimeZone,
{
    fn from(dt: DateTime<TZ>) -> Self {
        dt.signed_duration_since(Utc::now()).into()
    }
}

impl From<NaiveDateTime> for HumanTime {
    fn from(ndt: NaiveDateTime) -> Self {
        ndt.signed_duration_since(Utc::now().naive_utc()).into()
    }
}

impl Humanize for Duration {
    fn humanize(&self) -> String {
        format!("{}", HumanTime::from(*self))
    }
}

impl<TZ> Humanize for DateTime<TZ>
where
    TZ: TimeZone,
{
    fn humanize(&self) -> String {
        format!("{}", HumanTime::from(self.clone()))
    }
}

impl Humanize for NaiveDateTime {
    fn humanize(&self) -> String {
        format!("{}", HumanTime::from(*self))
    }
}

impl Humanize for usize {
    fn humanize(&self) -> String {
        ByteSize(*self as u64).to_string_as(true)
    }
}