use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Clock(u8, u8);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        fn modulo(n: i32, modulus: i32) -> i32 {
            let m = n % modulus;
            if m < 0 { m + modulus } else { m }
        }
        let m = modulo(minutes, 60);
        let h = modulo(hours + (minutes - m) / 60, 24);
        Clock(h as u8, m as u8)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.0 as i32, self.1 as i32 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, mut f: &mut fmt::Formatter) -> fmt::Result {
        write!(&mut f, "{:02}:{:02}", self.0, self.1)
    }
}
