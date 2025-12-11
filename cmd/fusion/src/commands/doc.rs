use anyhow::Result;
use fusion_docgen;

pub fn doc(open: bool, private: bool) -> Result<()> {
    let _path = fusion_docgen::generate(private)?;
    if open {
        println!(
            "Opening docs is not yet implemented in CLI. Path: {}",
            _path.display()
        );
    }
    Ok(())
}
