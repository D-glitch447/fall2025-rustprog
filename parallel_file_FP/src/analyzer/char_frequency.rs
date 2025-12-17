use crate::analyzer::Analyzer;
use crate::models::stats::FileStats;

pub struct CharFrequencyAnalyzer;

impl Analyzer for CharFrequencyAnalyzer {
    fn name(&self) -> &'static str {
        "char_frequency"
    }

    fn analyze(&self, content: &str, stats: &mut FileStats) -> Result<(), String> {
        for ch in content.chars() {
            *stats.char_frequencies.entry(ch).or_insert(0) += 1;
        }
        Ok(())
    }
}
