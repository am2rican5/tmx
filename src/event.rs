use std::time::{Duration, Instant};

use color_eyre::eyre::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    Resize,
}

pub struct EventReader {
    tick_rate: Duration,
    last_tick: Instant,
}

impl EventReader {
    pub fn new(tick_rate: Duration) -> Self {
        Self {
            tick_rate,
            last_tick: Instant::now(),
        }
    }

    pub fn next(&mut self) -> Result<AppEvent> {
        let timeout = self
            .tick_rate
            .checked_sub(self.last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        if event::poll(timeout)? {
            match event::read()? {
                CrosstermEvent::Key(key) => return Ok(AppEvent::Key(key)),
                CrosstermEvent::Resize(_, _) => return Ok(AppEvent::Resize),
                _ => {}
            }
        }

        if self.last_tick.elapsed() >= self.tick_rate {
            self.last_tick = Instant::now();
            return Ok(AppEvent::Tick);
        }

        Ok(AppEvent::Tick)
    }
}
