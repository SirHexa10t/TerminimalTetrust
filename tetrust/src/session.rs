use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct UserInput {
    pub sigterm: Arc<AtomicBool>,
}

impl Default for UserInput {
    fn default() -> Self {
        let is_sigterm = Arc::new(AtomicBool::new(false));
        let r = is_sigterm.clone();

        let _ = ctrlc::set_handler(move || {
            r.store(true, Ordering::SeqCst);
        });

        Self { sigterm: is_sigterm, }
    }
}


