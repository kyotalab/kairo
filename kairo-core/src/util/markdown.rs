use crate::interface::MarkdownParsable;
use crate::interface::{FrontMatterExportable, HasItem, MarkdownExportable};
use serde::Serialize;
use std::fs;
use std::io;

pub fn write_to_markdown<C, F, N>(item: &C, dir: &str) -> Result<(), io::Error>
where
    // NoteContent
    C: MarkdownExportable<F> + Serialize,
    // NoteFrontMatter
    F: FrontMatterExportable<N> + Serialize,
    N: HasItem,
{
    let serialized = serde_yaml::to_string(&item.get_front_matter()).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("YAML serialization failed: {e}"),
        )
    })?;

    let content = if let Some(body) = item.get_body() {
        format!("---\n{}---{}", serialized, body)
    } else {
        format!(
            "---\n{}---\n\n## {}\n\n",
            serialized,
            item.get_front_matter().get_item().title()
        )
    };

    let path = format!("{}/{}.md", dir, item.get_front_matter().get_item().id());

    fs::create_dir_all(dir)?;
    fs::write(&path, content)?;

    println!(
        "✅ Markdown saved to {}.md",
        item.get_front_matter().get_item().id()
    );
    Ok(())
}

// TODO ここにMarkdownをパースして、front_matterとbodyを取得する関数を実装する。
pub fn parse_markdown<T, U>(item: &T, dir: &str) -> Result<(String, String), io::Error>
where
    T: MarkdownParsable<U> + Serialize,
    U: HasItem,
{
    // use regex::Regex;

    let path = format!("{}/{}.md", dir, item.get_item().id());

    let content = fs::read_to_string(path)?;
    let re = String::from("---");

    // let re = Regex::new("---").unwrap();
    let splitted_contents: Vec<_> = content.split(&re).collect();
    let contents: Vec<_> = splitted_contents
        .into_iter()
        .map(|content| content.to_string())
        .collect();

    let note_content = (contents[1].clone(), contents[2].clone());
    Ok(note_content)
}
