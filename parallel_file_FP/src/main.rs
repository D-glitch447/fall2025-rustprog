use std::path::PathBuf;
use parallel_file_FP::processing::FileProcessor;

fn main() {
    let dirs = vec![PathBuf::from("./books")];

    let processor = FileProcessor::new(8);
    let results = processor.run_on_dirs(&dirs);

    for r in results {
        println!("{:#?}", r);
    }
}
