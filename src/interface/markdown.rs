// FrontMatterのフィールドを取得するためのトレイト
pub trait MarkdownExportable<U> {
    fn get_item(&self) -> &U;
    fn get_tags(&self) -> &[String];
}

// Markdownを生成するために必要なIDとTitleを取得するためのトレイト
pub trait HasItem {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
}
