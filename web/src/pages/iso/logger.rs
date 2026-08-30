use core::fmt;
use std::rc::Rc;

use chrono::Utc;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Logger {
    buffer: String,
    logs: Vec<Rc<str>>
}

impl Logger {
    pub const fn new() -> Self {
        Self {
            buffer: String::new(),
            logs: Vec::new(),
        }
    }

    pub fn log(&mut self, message: &str) {
        let now = Utc::now();
        let message = format!("{} {}", now.to_rfc3339(), message);
        self.logs.push(message.into());
    }

    pub fn clear(&mut self) {
        self.logs.clear();
        self.buffer.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.logs.is_empty() && self.buffer.is_empty()
    }

    pub fn len(&self) -> usize {
        self.logs.len()
    }

    pub fn into_inner(self) -> Vec<Rc<str>> {
        self.logs
    }

    pub fn flush(&mut self) {
        if !self.buffer.is_empty() {
            self.logs.push(self.buffer.clone().into());
            self.buffer.clear();
        }
    }

    pub fn entries(&self) -> &[Rc<str>] {
        &self.logs
    }
}

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.push_str(s);

        if s.contains('\n') {
            let mut buffer = core::mem::take(&mut self.buffer);
            let now = Utc::now();
            buffer.insert_str(0, &format!("{} ", now.to_rfc3339()));
            self.logs.push(buffer.into());
        }

        Ok(())
    }
}