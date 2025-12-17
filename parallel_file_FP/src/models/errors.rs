#[derive(Debug, Clone)]
pub enum ProcessingError {
    IoError { context: String, message: String },
    EncodingError { context: String, message: String },
    AnalyzerError { analyzer: String, message: String },
}
