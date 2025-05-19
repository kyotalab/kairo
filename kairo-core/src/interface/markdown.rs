pub trait MarkdownExportable<U> {
    fn get_front_matter(&self) -> &U;
    fn get_body(&self) -> &Option<String>;
}

pub trait FrontMatterExportable<U> {
    fn get_item(&self) -> &U;
    fn get_tags(&self) -> &[String];
}

pub trait HasItem {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
}

pub trait MarkdownParsable<U> {
    fn get_item(&self) -> &U;
}
