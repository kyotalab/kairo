use crate::interface::MarkdownParsable;
use crate::interface::{FrontMatterExportable, HasItem, MarkdownExportable};
use serde::Serialize;
use std::fs;
use std::io;

// この関数は、FrontMatterとBodyをマッピングした構造体をシリアライズして
// Markdown ファイルを生成する。
//
// 新規作成と更新の時に合わせて、条件を分岐させている。
// 新規作成の場合（note create）、BodyにはTitleのみ。
// 更新の場合（note update）、Bodyには書かれている内容で上書き。
pub fn write_to_markdown<C, F, I>(item: &C, dir: &str) -> Result<(), io::Error>
where
    // C(Content)
    C: MarkdownExportable<F> + Serialize,
    // F(FrontMatter)
    F: FrontMatterExportable<I> + Serialize,
    // I(Item)
    I: HasItem,
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

// この関数は Markdown 形式の文字列を解析して、
// FrontMatter と 本文を分割して返す。
//
// `---` を区切り文字として使うが、本文中にも現れる可能性があるため、
// 先頭から3つまでの分割に限定している。
pub fn parse_markdown<T, U>(item: &T, dir: &str) -> Result<(String, String), io::Error>
where
    T: MarkdownParsable<U> + Serialize,
    U: HasItem,
{
    let path = format!("{}/{}.md", dir, item.get_item().id());

    let content = fs::read_to_string(path)?;
    let re = String::from("---");

    let splitted_contents = content.splitn(3, &re);

    let contents: Vec<_> = splitted_contents
        .into_iter()
        .map(|content| content.to_string())
        .collect();

    let markdown_content = (contents[1].clone(), contents[2].clone());
    Ok(markdown_content)
}
