/// Pure Rust implementation of the counter.
#[derive(Debug, Clone)]
pub struct Counter {
    value: i64,
    label: Option<String>,
}

impl Counter {
    pub fn new(initial: i64) -> Self {
        Self { value: initial, label: None }
    }

    pub fn increment(&mut self, delta: i64) {
        self.value = self.value.saturating_add(delta);
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }

    pub fn set_label(&mut self, s: Option<String>) {
        self.label = s;
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}
