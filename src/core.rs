use std::fmt;

/// Pure Rust implementation of the counter.
pub struct Counter {
    value: i64,
    label: Option<String>,
    callback: Option<Box<dyn FnMut(i64)>>,
}

impl Counter {
    pub fn new(initial: i64) -> Self {
        Self {
            value: initial,
            label: None,
            callback: None,
        }
    }

    pub fn increment(&mut self, delta: i64) {
        self.value = self.value.saturating_add(delta);
        if let Some(callback) = &mut self.callback {
            callback(self.value);
        }
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0;
        if let Some(callback) = &mut self.callback {
            callback(self.value);
        }
    }

    pub fn set_label(&mut self, s: Option<String>) {
        self.label = s;
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn set_callback(&mut self, callback: Option<Box<dyn FnMut(i64)>>) {
        self.callback = callback;
    }
}

impl fmt::Debug for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Counter")
            .field("value", &self.value)
            .field("label", &self.label)
            .finish_non_exhaustive()
    }
}

impl Clone for Counter {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            label: self.label.clone(),
            callback: None,
        }
    }
}


