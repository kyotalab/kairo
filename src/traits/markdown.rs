pub trait MarkdownExportable {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
}
