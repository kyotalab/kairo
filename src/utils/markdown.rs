use crate::models::Note;
use std::fs;
use std::io;

pub fn write_note_to_markdown(note: &Note, dir: &str) -> Result<(), io::Error> {
    let serialized_note = serde_yaml::to_string(&note).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("YAML serialization failed: {e}"),
        )
    })?;

    let frontmatter = format!("---\n{}---\n\n## {}\n\n", serialized_note, note.title);
    let file_path = format!("{}/{}.md", dir, note.id);

    fs::create_dir_all(dir)?; // フォルダがない場合に作成
    fs::write(&file_path, frontmatter)?;

    println!("✅ Markdown saved to {}.md", note.id);
    Ok(())
}
