use std::fmt;
    
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Clock{
    time: i32,
}

impl Clock {
    const minutes_in_a_day: i32 = 24*60;
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{
            time: (60*hours+minutes).rem_euclid(Self::minutes_in_a_day),
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.time += minutes;
        self.time = self.time.rem_euclid(Self::minutes_in_a_day);
        *self
    }
}

impl fmt::Display for Clock{
    fn fmt(&self,f: &mut fmt::Formatter<'_>) ->fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.time/60, self.time%60)
    }
}
impl PartialEq for Clock{
    fn eq (&self, other: &Self) ->bool{
        self.time == other.time
    }
}