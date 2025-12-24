use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct ProgressManager {
    pub progress_bar: ProgressBar,
    pub semaphore: Arc<Semaphore>,
}

impl ProgressManager {
    pub fn new(total: u64) -> Self {
        let progress_bar = ProgressBar::new(total);
        
        // Use expect to handle the template result
        let style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .expect("Invalid template")
            .progress_chars("#>-");
        
        progress_bar.set_style(style);
        
        let semaphore = Arc::new(Semaphore::new(10)); // Default to 10 concurrent permits
        
        Self {
            progress_bar,
            semaphore,
        }
    }
    
    pub fn set_concurrent_limit(&self, _limit: usize) {
        // This would require changing the semaphore, which we can't do after creation
        // So we'll create a new one and replace it in the caller
    }
    
    pub fn finish(&self) {
        self.progress_bar.finish_with_message("Done");
    }
}
