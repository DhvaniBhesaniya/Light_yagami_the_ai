use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Loader {
    message: String,
    running: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Loader {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            running: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    pub fn start(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let message = self.message.clone();

        self.handle = Some(thread::spawn(move || {
            let mut visible = true;
            while running.load(Ordering::SeqCst) {
                print!("\r\x1b[K"); // Clear line
                if visible {
                    print!("<< AI --->: {}", message);
                } else {
                    print!("<< AI --->:");
                }
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(500));
                visible = !visible;
            }
            // Clean up line when stopped
            print!("\r\x1b[K");
            io::stdout().flush().unwrap();
        }));
    }

    pub fn stop(&mut self) {
        if !self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}
