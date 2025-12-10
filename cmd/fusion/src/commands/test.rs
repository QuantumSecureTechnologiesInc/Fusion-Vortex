use anyhow::Result;
use fusion_tester;

pub fn test(filter: Option<&str>, release: bool, bench: bool) -> Result<()> {
    fusion_tester::test_project(filter, release, bench)
}
