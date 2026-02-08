#[derive(Debug, Clone)]
pub struct PwdState {
    current_dir: String,
    old_dir: String,
}

impl PwdState {
    pub fn new(current_dir: String, old_dir: String) -> Self {
        Self {
            current_dir,
            old_dir,
        }
    }
    pub fn set_states(&mut self, new_current: String, new_old: String) {
        self.current_dir = new_current;
        self.old_dir = new_old;
    }

    pub fn get_current_dir(&self) -> String {
        self.current_dir.clone()
    }

    pub fn get_old_dir(&self) -> String {
        self.old_dir.clone()
    }
}
