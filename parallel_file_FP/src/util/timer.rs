use std::time::Instant;

pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn start() -> Self {
        Self { start: Instant::now() }
    }

    pub fn elapsed(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
}
