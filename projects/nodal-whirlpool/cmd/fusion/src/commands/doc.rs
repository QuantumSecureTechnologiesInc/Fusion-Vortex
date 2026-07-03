use anyhow::Result;
use fusion_docgen;

pub fn doc(open: bool, private: bool) -> Result<()> {
    fusion_docgen::generate(open, private)
}
