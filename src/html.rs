use log::debug;
use log::warn;

use crate::intermediary::Block;
use crate::intermediary::RichText;

pub fn from_blocks(blocks: Vec<Block>, extra: bool) -> String {
    let mut out = String::new();

    for block in preprocess(blocks) {
        if extra {
            debug!("extra type is {}", block.to_string())
        }
        match block {
            Block::Header { rich_text, size } => {
                out += &format!("<h{}>{}</h{}>", size, rich_text_to_html(rich_text), size);
            }
            Block::Divider => out += "<hr />",
            Block::Quote {
                rich_text,
                children,
            } => {
                out += &format!("<blockquote>{}</blockquote>", rich_text_to_html(rich_text));
                if let Some(children) = children {
                    out += &from_blocks(children, false);
                }
            }
            Block::CodeBlock { text, lang } => {
                out += &format!(
                    "<pre><code class=\"language-{}\">{}</code></pre>",
                    lang, text
                );
            }
            Block::List { items } => {
                debug!("LIST HTML LIST");
                out += "<ul>";
                for item in items {
                    debug!("parse");
                    debug!("{:#?}", item);
                    let h = from_blocks(vec![item], true);
                    debug!("end parse");
                    out += &format!("<li>{}</li>", h);
                }
                out += "</ul>";
            }
            Block::NumberedList { items } => {
                debug!("LIST HTML LIST");
                out += "<ol>";
                for item in items {
                    debug!("parse");
                    debug!("{:#?}", item);
                    let h = from_blocks(vec![item], true);
                    debug!("end parse");
                    out += &format!("<li>{}</li>", h);
                }
                out += "</ol>";
            }
            Block::TodoList { items: list } => {
                out += "<ul>";
                for (checked, item) in list {
                    out += &format!(
                        "<li><input type=\"checkbox\" {}>{}</li>",
                        if checked { "checked" } else { "" },
                        from_blocks(vec![item], false)
                    );
                }
                out += "</ul>";
            }
            Block::Line { rich_text } => {
                if rich_text.is_empty() {
                    out += "<br />";
                } else {
                    if extra {
                        debug!("{:#?}", rich_text);
                    }
                    out += &format!("<p>{}</p>", rich_text_to_html(rich_text));
                }
            }
            _ => warn!("Can't find html block type for {:?}", block.to_string()),
        }
    }

    out
}

fn preprocess(blocks: Vec<Block>) -> Vec<Block> {
    let mut out = vec![];
    // temporary placeholder
    let mut last_block = Block::Empty;

    for block in blocks {
        match block {
            Block::Header { rich_text, size } => {
                out.push(last_block);
                last_block = Block::Header { rich_text, size };
            }
            Block::Divider => {
                out.push(last_block);
                last_block = Block::Divider;
            }
            Block::Quote {
                rich_text,
                children: _,
            } => {
                out.push(last_block);
                last_block = Block::Quote {
                    rich_text,
                    children: None,
                };
            }
            Block::CodeBlock { text, lang } => {
                out.push(last_block);
                last_block = Block::CodeBlock { text, lang };
            }
            Block::List { items } => match last_block {
                Block::List { items: last_items } => {
                    let mut new_items = last_items;
                    new_items.extend(items);
                    last_block = Block::List { items: new_items };

                    debug!("{:#?}", last_block);
                }
                _ => {
                    out.push(last_block);
                    last_block = Block::List { items };
                }
            },
            Block::NumberedList { items } => match last_block {
                Block::NumberedList { items: last_items } => {
                    let mut new_items = last_items;
                    new_items.extend(items);
                    last_block = Block::NumberedList { items: new_items };

                    debug!("{:#?}", last_block);
                }
                _ => {
                    out.push(last_block);
                    last_block = Block::NumberedList { items };
                }
            },
            Block::TodoList { items } => {
                out.push(last_block);
                last_block = Block::TodoList { items };
            }
            Block::Line { rich_text } => {
                if rich_text.is_empty() {
                    out.push(last_block);
                    last_block = Block::Line { rich_text };
                } else {
                    match last_block {
                        Block::Line {
                            rich_text: last_rich_text,
                        } => {
                            let mut new_rich_text = last_rich_text;
                            new_rich_text.push(RichText::default());
                            new_rich_text.extend(rich_text);
                            last_block = Block::Line {
                                rich_text: new_rich_text,
                            };
                        }
                        _ => {
                            out.push(last_block);
                            last_block = Block::Line { rich_text };
                        }
                    }
                }
            }
            _ => warn!(
                "Can't find intermediary block type for {:?}",
                block.to_string()
            ),
        }
    }

    out.push(last_block);

    out
}

fn rich_text_to_html(rich_text: Vec<RichText>) -> String {
    let mut out = String::new();
    for text in rich_text {
        if text == RichText::default() {
            out += "<br />";
            continue;
        }
        let mut tags = vec![];
        if text.bold {
            tags.push("b");
        }
        if text.italic {
            tags.push("i");
        }
        if text.underline {
            tags.push("u");
        }
        if text.strikethrough {
            tags.push("s");
        }
        if text.code {
            tags.push("code");
        }

        let output = &format!(
            "{}{}{}",
            tags.iter().map(|t| format!("<{}>", t)).collect::<String>(),
            text.plain_text,
            tags.iter().map(|t| format!("</{}>", t)).collect::<String>()
        );

        match text.href {
            Some(href) => out += &format!("<a href=\"{}\">{}</a>", href, output),
            None => out += output,
        }
    }
    out
}
