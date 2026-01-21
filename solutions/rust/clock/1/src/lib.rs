use std::fmt;

const MINUTES_IN_DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq)]
pub struct Clock(String);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        Self::from_minutes(total_minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let current_minutes = self.to_minutes();
        Self::from_minutes(current_minutes + minutes)
    }

    fn from_minutes(total_minutes: i32) -> Self {
        let normalized =
            ((total_minutes % MINUTES_IN_DAY) + MINUTES_IN_DAY) % MINUTES_IN_DAY;

        let hours = normalized / 60;
        let minutes = normalized % 60;

        Clock(format!("{:02}:{:02}", hours, minutes))
    }

    fn to_minutes(&self) -> i32 {
        let mut parts = self.0.split(':');
        let hours: i32 = parts.next().unwrap().parse().unwrap();
        let minutes: i32 = parts.next().unwrap().parse().unwrap();
        hours * 60 + minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
