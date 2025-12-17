use crate::analyzer::Analyzer;
use crate::models::stats::FileStats;

pub struct LineCountAnalyzer;

impl Analyzer for LineCountAnalyzer {
    fn name(&self) -> &'static str {
        "line_count"
    }

    fn analyze(&self, content: &str, stats: &mut FileStats) -> Result<(), String> {
        stats.line_count = content.lines().count();
        Ok(())
    }
}
