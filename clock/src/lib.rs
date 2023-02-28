use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total = (hours*60 + minutes) % 1440;
        let total = if total < 0 {
            let mut a = total;
            while a < 0 {
                a = a + 1440;
            }
            a
        }
        else {total};
        Clock {
            hours: (total / 60) % 24,
            minutes: total % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}