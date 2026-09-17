use std::fmt;
    
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Clock{
    time: i32,
}

impl Clock {
    const MINUTES_IN_A_DAY: i32 = 24*60;
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{
            time: (60*hours+minutes).rem_euclid(Self::MINUTES_IN_A_DAY),
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Self::new(0, self.time + minutes)
    }
}

impl fmt::Display for Clock{
    fn fmt(&self,f: &mut fmt::Formatter<'_>) ->fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.time/60, self.time%60)
    }
}