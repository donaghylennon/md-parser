use core::iter::Peekable;
use core::str::Chars;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
enum MarkdownText {
    Text(String),
    Bold(Vec<MarkdownText>),
    Italic(Vec<MarkdownText>),
}

#[derive(Debug)]
enum Markdown {
    Heading(u8, Vec<MarkdownText>),
    Paragraph(Vec<MarkdownText>),
    BlockQuote(Vec<MarkdownText>),
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
        println!("heading.md contains:\n{:?}", md_file);
    } else {
        println!("Something went wrong!");
    }

    if let Some(md_file) = parse_md_from_file("examples/bold_italic.md") {
        println!("bold_italic.md contains:\n{:?}", md_file);
    } else {
        println!("Something went wrong!");
    }

    if let Some(md_file) = parse_md_from_file("examples/paragraph.md") {
        println!("paragraph.md contains:\n{:?}", md_file);
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
                md_file.push(parse_paragraph(&mut chars));
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
    let text: String = chars
        .skip_while(|&c| c.is_whitespace())
        .take_while(|&c| c != '\n')
        .collect();
    Markdown::Heading(num, parse_md_text(&mut text.chars().peekable()))
}

fn parse_paragraph(chars: &mut Peekable<Chars>) -> Markdown {
    let mut prev_was_newline = false;
    let text: String = chars
        .take_while(|&c| {
            if prev_was_newline {
                prev_was_newline = false;
                if c == '\n' {
                    false
                } else {
                    true
                }
            } else {
                if c == '\n' {
                    prev_was_newline = true;
                }
                true
            }
        })
        .collect();

    Markdown::Paragraph(parse_md_text(&mut text.chars().peekable()))
}

fn parse_md_text(chars: &mut Peekable<Chars>) -> Vec<MarkdownText> {
    let mut md_text = vec![];
    let mut current_text: Option<String> = None;
    while let Some(&ch) = chars.peek() {
        match ch {
            '*' => {
                if let Some(text) = current_text.take() {
                    md_text.push(MarkdownText::Text(text));
                }
                chars.next();
                md_text.push(parse_md_text_bold(chars));
            }
            '_' => {
                if let Some(text) = current_text.take() {
                    md_text.push(MarkdownText::Text(text));
                }
                chars.next();
                md_text.push(parse_md_text_italic(chars));
            }
            _   => {
                if let Some(text) = &mut current_text {
                    text.push(ch);
                } else {
                    let mut new_string = String::new();
                    new_string.push(ch);
                    current_text = Some(new_string);
                }
                chars.next();
            }
        }
    }
    if let Some(text) = current_text.take() {
        md_text.push(MarkdownText::Text(text));
    }
    md_text
}

fn parse_md_text_bold(chars: &mut Peekable<Chars>) -> MarkdownText {
    let mut md_text = vec![];
    let text: String = chars
        .take_while(|&c| c != '*')
        .collect();
    let mut chars = text.chars()
        .peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '_' => {
                chars.next();
                md_text.push(parse_md_text_italic(&mut chars));
            }
            _   => {
                md_text.push(parse_md_text_text(&mut chars));
            }
        }
    }
    MarkdownText::Bold(md_text)
}

fn parse_md_text_italic(chars: &mut Peekable<Chars>) -> MarkdownText {
    let mut md_text = vec![];
    let text: String = chars
        .take_while(|&c| c != '_')
        .collect();
    let mut chars = text.chars()
        .peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '*' => {
                chars.next();
                md_text.push(parse_md_text_bold(&mut chars));
            }
            _   => {
                md_text.push(parse_md_text_text(&mut chars));
            }
        }
    }
    MarkdownText::Italic(md_text)
}

fn parse_md_text_text(chars: &mut Peekable<Chars>) -> MarkdownText {
    MarkdownText::Text(chars.take_while(|&c| c != '*' && c != '_').collect())
}
