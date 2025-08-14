use jiff::{Span, ToSpan};

type R = light_ranged_integers::RangedU8<0, 23>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Hour(R);

impl Hour {
    pub fn new(n: u8) -> Self {
        Self(R::new(n))
    }

    pub fn new_try(n: u8) -> Option<Self> {
        R::new_try(n).map(Self)
    }

    fn into_i32(self) -> i32 {
        i32::from(self.0.inner())
    }
}

impl From<Hour> for i32 {
    fn from(value: Hour) -> Self {
        value.into_i32()
    }
}

impl From<Hour> for u8 {
    fn from(value: Hour) -> Self {
        value.0.inner()
    }
}

impl ToSpan for Hour {
    fn years(self) -> Span {
        self.into_i32().years()
    }

    fn months(self) -> Span {
        self.into_i32().months()
    }

    fn weeks(self) -> Span {
        self.into_i32().weeks()
    }

    fn days(self) -> Span {
        self.into_i32().days()
    }

    fn hours(self) -> Span {
        self.into_i32().hours()
    }

    fn minutes(self) -> Span {
        self.into_i32().minutes()
    }

    fn seconds(self) -> Span {
        self.into_i32().seconds()
    }

    fn milliseconds(self) -> Span {
        self.into_i32().milliseconds()
    }

    fn microseconds(self) -> Span {
        self.into_i32().microseconds()
    }

    fn nanoseconds(self) -> Span {
        self.into_i32().nanoseconds()
    }
}
