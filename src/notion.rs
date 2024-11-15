use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PageResponse {
    //pub object: String,
    pub results: Vec<Block>,
    //pub next_cursor: Option<String>,
    //pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    //pub object: String,
    //pub id: String,
    //parent: Parent,
    //#[serde(rename = "created_time")]
    //pub created: String,
    //#[serde(rename = "last_edited_time")]
    //pub last_edited: String,
    // created_by: {object, id}
    // last_edited_by: {object, id}
    //pub has_children: bool,
    //pub archived: bool,
    //pub in_trash: bool,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(flatten)]
    pub block: BlockData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockData {
    Bookmark {
        //caption: Vec<RichText>,
        //url: String,
    },
    Breadcrumb,
    BulletedListItem {
        rich_text: Vec<RichText>,
        //color: NotionColor,
        children: Option<Vec<Block>>,
    },
    Callout {
        //rich_text: Vec<RichText>,
        // icon (emoji or file)
        //color: NotionColor,
    },
    ChildDatabase {
        //title: String,
    },
    ChildPage {
        //title: String,
    },
    Code {
        //caption: Vec<RichText>,
        rich_text: Vec<RichText>,
        language: NotionLanguages,
    },
    ColumnList,
    Column,
    Divider,
    Embed {
        //url: String,
    },
    Equation {
        //expression: String,
    },
    File {
        // TODO: file
        //caption: Vec<RichText>,
        //#[serde(rename = "type")]
        //ty: String,
        //name: String,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        rich_text: Vec<RichText>,
        //color: NotionColor,
        //is_toggleable: bool,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        rich_text: Vec<RichText>,
        //color: NotionColor,
        //is_toggleable: bool,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        rich_text: Vec<RichText>,
        //color: NotionColor,
        //is_toggleable: bool,
    },
    Image {
        //#[serde(rename = "type")]
        //ty: String,
        // TODO: image
    },
    LinkPreview {
        //url: String,
    },
    Mention(/* MentionData */),
    NumberedListItem {
        rich_text: Vec<RichText>,
        //color: NotionColor,
        children: Option<Vec<Block>>,
    },
    Paragraph {
        rich_text: Vec<RichText>,
        //color: NotionColor,
        //children: Option<Vec<Block>>,
    },
    Pdf {
        // TODO: pdf
    },
    Quote {
        rich_text: Vec<RichText>,
        //color: NotionColor,
        children: Option<Vec<Block>>,
    },
    // synced block
    Table {
        //table_width: usize,
        //has_column_header: bool,
        //has_column_totals: bool,
    },
    TableRow {
        //cells: Vec<RichText>,
    },
    TableOfContents {
        //color: NotionColor,
    },
    ToDo {
        rich_text: Vec<RichText>,
        checked: bool,
        //color: NotionColor,
        children: Option<Vec<Block>>,
    },
    Toggle {
        //rich_text: Vec<RichText>,
        //color: NotionColor,
        //children: Option<Vec<Block>>,
    },
    Video {
        // TODO: file
    },
}

#[derive(Debug, Deserialize)]
pub struct RichText {
    //#[serde(rename = "type")]
    //pub ty: String,
    //#[serde(flatten)]
    //pub data: RichTextData,
    pub annotations: Annotations,
    pub plain_text: String,
    pub href: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: NotionColor,
}

/*#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RichTextData {
    Text {
        content: String,
        link: Option<Url>,
    },
    Equation {
        expression: String,
    },
    Mention {
        #[serde(rename = "type")]
        ty: String,
        #[serde(flatten)]
        data: MentionData,
    },
}*/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotionLanguages {
    Abap,
    Arduino,
    Bash,
    Basic,
    C,
    Clojure,
    Coffeescript,
    #[serde(rename = "c++")]
    Cpp,
    #[serde(rename = "c#")]
    CSharp,
    Css,
    Dart,
    Diff,
    Docker,
    Elixir,
    Elm,
    Erlang,
    Flow,
    Fortran,
    #[serde(rename = "f#")]
    FSharp,
    Gherkin,
    Glsl,
    Go,
    GraphQl,
    Groovy,
    Haskell,
    Html,
    Java,
    Javascript,
    Json,
    Julia,
    Kotlin,
    Latex,
    Less,
    Lisp,
    Liverscript,
    Lua,
    Makefile,
    Markdown,
    Markup,
    Matlab,
    Mermaid,
    Nix,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    Ocaml,
    Pascal,
    Perl,
    Php,
    #[serde(rename = "plain text")]
    PlainText,
    Powershell,
    Prolog,
    Protobuf,
    Python,
    R,
    Reason,
    Ruby,
    Rust,
    Sass,
    Scala,
    Scheme,
    Scss,
    Shell,
    Sql,
    Swift,
    Typescript,
    #[serde(rename = "vb.net")]
    VbNet,
    Verilog,
    Vhdl,
    #[serde(rename = "visual basic")]
    VisualBasic,
    WebAssembly,
    Xml,
    Yaml,
    #[serde(rename = "java/c/c++/c#")]
    JavaOrCLangs,
}

impl Display for NotionLanguages {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/*#[derive(Debug, Deserialize)]
pub struct Url {
    pub url: String,
}*/

/*#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MentionData {
    Database {
        id: String,
    },
    Date {
        start: String,
        end: Option<String>,
    },
    LinkPreview {
        url: String,
    },
    Page {
        id: String,
    },
    TemplateMention {
        #[serde(rename = "type")]
        ty: String,
        template_mention_date: Option<String>,
        // we dont need the other one because its always "me" and we can tell from the type
    },
    User {
        object: String,
        id: String,
    },
}*/

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotionColor {
    Blue,
    BlueBackground,
    Brown,
    BrownBackground,
    Default,
    Gray,
    GrayBackground,
    Green,
    GreenBackground,
    Orange,
    OrangeBackground,
    Yellow,
    YellowBackground,
    Pink,
    PinkBackground,
    Purple,
    PurpleBackground,
    Red,
    RedBackground,
}
