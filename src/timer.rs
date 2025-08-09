pub struct Timer {
    pub remaining: f32,
}

impl Timer {
    pub fn new(seconds: u32) -> Self {
        Self { remaining: seconds as f32 }
    }

    pub fn tick(&mut self, delta: f32) {
        if self.remaining > 0.0 {
            self.remaining -= delta;
        }
    }

    pub fn reset(&mut self, seconds: u32) {
        self.remaining = seconds as f32;
    }

    pub fn formatted(&self) -> String {
        let minutes = (self.remaining / 60.0).floor() as u32;
        let seconds = (self.remaining % 60.0).floor() as u32;
        format!("{:02}:{:02}", minutes, seconds)
    }
}