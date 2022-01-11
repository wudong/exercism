#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {   
        let mut hours_from_minutes = minutes / 60;
        let mut minutes_in_range = minutes % 60;
        if minutes_in_range < 0 {
            minutes_in_range = minutes_in_range + 60;
            hours_from_minutes = hours_from_minutes -1;
        }
        
        let mut hours_in_range = (hours + hours_from_minutes) % 24;
        if hours_in_range < 0 {
            hours_in_range = hours_in_range + 24;
        }

        Self {
            hours: hours_in_range,
            minutes: minutes_in_range,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
       Clock::new(self.hours, self.minutes+minutes)
    }
}

impl ToString for Clock {
    fn to_string(&self)-> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}