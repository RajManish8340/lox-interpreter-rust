use std::sync::Mutex;

pub(crate) static HAS_ERRORS: Mutex<bool> = Mutex::new(false);
