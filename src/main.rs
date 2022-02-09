use core::iter::Peekable;
use core::str::Chars;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
enum MarkdownText {
    Text(String),
    Bold(Box<MarkdownText>),
    Italic(Box<MarkdownText>),
}

#[derive(Debug)]
enum Markdown {
    Heading(u8, Box<MarkdownText>),
    Paragraph(Box<MarkdownText>),
    BlockQuote(Box<MarkdownText>),
    List(ListType, Vec<MarkdownText>),
    Code(String),
    HorizontalRule,
}

#[derive(Debug)]
enum ListType {
    Ordered,
    Unordered,
}

type MarkdownFile = Vec<Markdown>;

fn main() {
    if let Some(md_file) = parse_md_from_file("examples/heading.md") {
        println!("File contains:\n{:?}", md_file);
    } else {
        println!("Something went wrong!");
    }
}

fn parse_md_from_file(filename: &str) -> Option<MarkdownFile> {
    let path = Path::new(filename);
    let mut file = File::open(&path).expect(&format!("Could not open {}", filename));

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(&format!("Could not read {} to string", filename));
    parse_md(&contents)
}

fn parse_md(input: &str) -> Option<MarkdownFile> {
    let mut md_file = vec![];
    let mut chars = input.chars().peekable();
    while let Some(&ch) = chars.peek() {
        match ch {
            '#' => {
                md_file.push(parse_heading(&mut chars));
            }
            '\n' => {
                chars.next();
            }
            _ => {
                chars.next();
            }
        };
    }
    Some(md_file)
}

fn parse_heading(chars: &mut Peekable<Chars>) -> Markdown {
    let mut num = 0;
    while *chars.peek().unwrap() == '#' {
        chars.next();
        num += 1;
    }
    let text = chars
        .skip_while(|&c| c.is_whitespace())
        .take_while(|&c| c != '\n')
        .collect();
    Markdown::Heading(num, Box::new(MarkdownText::Text(text)))
}
