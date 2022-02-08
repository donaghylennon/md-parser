enum MarkdownText {
    Text(String),
    Bold(Box<MarkdownText>),
    Italic(Box<MarkdownText>),
}

enum Markdown {
    Heading(u8, Box<MarkdownText>),
    Paragraph(Box<MarkdownText>),
    BlockQuote(Box<MarkdownText>),
    List(ListType, Vec<MarkdownText>),
    Code(String),
    HorizontalRule,
}

enum ListType {
    Ordered, Unordered
}

type MarkdownFile = Vec<Markdown>;

fn main() {
    println!("Hello, world!");
}
