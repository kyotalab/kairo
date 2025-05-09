use crate::traits::markdown::MarkdownExportable;
use serde::Serialize;
use std::fs;
use std::io;

pub fn write_to_markdown<T>(item: &T, dir: &str) -> Result<(), io::Error>
where
    T: MarkdownExportable + Serialize,
{
    let serialized = serde_yaml::to_string(&item).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("YAML serialization failed: {e}"),
        )
    })?;

    let content = format!("---\n{}---\n\n## {}\n\n", serialized, item.title());
    let path = format!("{}/{}.md", dir, item.id());

    fs::create_dir_all(dir)?;
    fs::write(&path, content)?;

    println!("✅ Markdown saved to {}.md", item.id());
    Ok(())
}
