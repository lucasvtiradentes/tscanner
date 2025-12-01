use globset::{Glob, GlobSet, GlobSetBuilder};

pub fn compile_globset(patterns: &[String]) -> Result<GlobSet, Box<dyn std::error::Error>> {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let glob = Glob::new(pattern)?;
        builder.add(glob);
    }

    Ok(builder.build()?)
}

pub fn compile_optional_globset(
    patterns: &[String],
) -> Result<Option<GlobSet>, Box<dyn std::error::Error>> {
    if patterns.is_empty() {
        Ok(None)
    } else {
        Ok(Some(compile_globset(patterns)?))
    }
}
