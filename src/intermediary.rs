use std::default;
use std::fmt::Display;

use log::debug;
use log::error;
use log::warn;

use crate::notion::Block as NotionBlock;
use crate::notion::BlockData as NotionBlockData;
use crate::notion::NotionColor;
use crate::notion::RichText as NotionRichText;

pub fn parse_blocks(notion: Vec<NotionBlock>) -> Vec<Block> {
    let mut out = Vec::new();
    for block in notion {
        match block.block {
            // TODO: not ignore color?
            NotionBlockData::Heading1 { rich_text, .. } => out.push(Block::Header {
                rich_text: notion_to_text(rich_text),
                size: 1,
            }),
            NotionBlockData::Heading2 { rich_text, .. } => out.push(Block::Header {
                rich_text: notion_to_text(rich_text),
                size: 2,
            }),
            NotionBlockData::Heading3 { rich_text, .. } => out.push(Block::Header {
                rich_text: notion_to_text(rich_text),
                size: 3,
            }),
            NotionBlockData::Quote {
                rich_text,
                children,
                ..
            } => {
                if let Some(children) = children {
                    out.push(Block::Quote {
                        rich_text: notion_to_text(rich_text),
                        children: Some(parse_blocks(children)),
                    });
                } else {
                    out.push(Block::Quote {
                        rich_text: notion_to_text(rich_text),
                        children: None,
                    });
                }
            }
            NotionBlockData::Code {
                rich_text,
                language,
                ..
            } => out.push(Block::CodeBlock {
                text: rich_text.iter().map(|t| t.plain_text.clone()).collect(),
                lang: language.to_string(),
            }),
            NotionBlockData::ToDo {
                rich_text,
                checked,
                color,
                children,
            } => {
                if let Some(children) = children {
                    out.push(Block::TodoList {
                        items: vec![(
                            checked,
                            Block::Line {
                                rich_text: notion_to_text(rich_text),
                            },
                        )],
                    });
                    debug!("Todo children: {:?}", children);
                    warn!("Ignoring children of todo block");
                } else {
                    out.push(Block::TodoList {
                        items: vec![(
                            checked,
                            Block::Line {
                                rich_text: notion_to_text(rich_text),
                            },
                        )],
                    });
                }
            }
            NotionBlockData::BulletedListItem {
                rich_text,
                children,
                ..
            } => {
                if let Some(children) = children {
                    out.push(Block::List {
                        items: vec![Block::Line {
                            rich_text: notion_to_text(rich_text),
                        }],
                    });
                    debug!("List children: {:?}", children);
                    warn!("Ignoring children of list block");
                } else {
                    out.push(Block::List {
                        items: vec![Block::Line {
                            rich_text: notion_to_text(rich_text),
                        }],
                    });
                }
            }
            NotionBlockData::NumberedListItem {
                rich_text,
                children,
                ..
            } => {
                if let Some(children) = children {
                    out.push(Block::List {
                        items: vec![Block::Line {
                            rich_text: notion_to_text(rich_text),
                        }],
                    });
                    debug!("List children: {:?}", children);
                    warn!("Ignoring children of list block");
                } else {
                    out.push(Block::NumberedList {
                        items: vec![Block::Line {
                            rich_text: notion_to_text(rich_text),
                        }],
                    });
                }
            }
            NotionBlockData::Divider => out.push(Block::Divider),
            NotionBlockData::Paragraph { rich_text, .. } => out.push(Block::Line {
                rich_text: notion_to_text(rich_text),
            }),
            _ => warn!("Can't find intermediary block type for {:?}", block.ty),
        };
    }

    out
}

fn notion_to_text(text: Vec<NotionRichText>) -> Vec<RichText> {
    let mut out = Vec::new();
    for t in text {
        out.push(RichText {
            plain_text: t.plain_text,
            bold: t.annotations.bold,
            italic: t.annotations.italic,
            underline: t.annotations.underline,
            strikethrough: t.annotations.strikethrough,
            code: t.annotations.code,
            href: t.href,
            color: IntermediaryColor::from(t.annotations.color),
        })
    }

    out
}

#[derive(Debug)]
pub enum Block {
    Header {
        rich_text: Vec<RichText>,
        size: usize,
    },
    Divider,
    Quote {
        rich_text: Vec<RichText>,
        children: Option<Vec<Block>>,
    },
    CodeBlock {
        text: String,
        lang: String,
    },
    Image {
        url: String,
    },
    List {
        items: Vec<Block>,
    },
    NumberedList {
        items: Vec<Block>,
    },
    TodoList {
        items: Vec<(bool, Block)>,
    },
    Line {
        rich_text: Vec<RichText>,
    },
    Empty,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Block::Header { .. } => "Header",
                Block::Divider { .. } => "Divider",
                Block::Quote { .. } => "Quote",
                Block::CodeBlock { .. } => "CodeBlock",
                Block::Image { .. } => "Image",
                Block::List { .. } => "List",
                Block::NumberedList { .. } => "NumberedList",
                Block::TodoList { .. } => "TodoList",
                Block::Line { .. } => "Line",
                Block::Empty { .. } => "Empty",
            }
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RichText {
    pub plain_text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub code: bool,
    pub color: IntermediaryColor,
    pub href: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum IntermediaryColor {
    Blue,
    Brown,
    #[default]
    Default,
    Gray,
    Green,
    Orange,
    Yellow,
    Pink,
    Purple,
    Red,
}

impl From<NotionColor> for IntermediaryColor {
    fn from(value: NotionColor) -> Self {
        match value {
            NotionColor::Blue => IntermediaryColor::Blue,
            NotionColor::Brown => IntermediaryColor::Brown,
            NotionColor::Gray => IntermediaryColor::Gray,
            NotionColor::Green => IntermediaryColor::Green,
            NotionColor::Orange => IntermediaryColor::Orange,
            NotionColor::Yellow => IntermediaryColor::Yellow,
            NotionColor::Pink => IntermediaryColor::Pink,
            NotionColor::Purple => IntermediaryColor::Purple,
            NotionColor::Red => IntermediaryColor::Red,
            _ => IntermediaryColor::Default,
        }
    }
}
