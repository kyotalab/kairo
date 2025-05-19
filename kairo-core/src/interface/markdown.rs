// FrontMatterのフィールドを取得するためのトレイト
pub trait MarkdownExportable<U> {
    fn get_front_matter(&self) -> &U;
    fn get_body(&self) -> &Option<String>;
}

// FrontMatterのフィールドを取得するためのトレイト
pub trait FrontMatterExportable<U> {
    fn get_item(&self) -> &U;
    fn get_tags(&self) -> &[String];
}

// Markdownを生成するために必要なIDとTitleを取得するためのトレイト
pub trait HasItem {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
}

pub trait MarkdownParsable<U> {
    fn get_item(&self) -> &U;
}
