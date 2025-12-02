use dioxus::signals::{Signal, WritableExt};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LoginState {
    pub logged_in: Signal<bool>,
    pub error: Signal<Option<String>>,
}

impl LoginState {
    pub fn logged_in(&self) -> Signal<bool> {
        self.logged_in
    }

    pub fn errored(&self) -> Signal<Option<String>> {
        self.error
    }

    pub fn set(&mut self, logged_in: bool) {
        self.logged_in.set(logged_in);
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error.set(error);
    }
}
