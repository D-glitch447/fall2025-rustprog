use crate::models::FileStats;

pub trait Analyzer: Send + Sync {
    fn name(&self) -> &'static str;
    fn analyze(&self, content: &str, stats: &mut FileStats) -> Result<(), String>;
}
