use std::io::{stdout, Stdout, Write};

use crossterm::{
    execute,
    style::{
        Attribute, Color, ContentStyle, Print, ResetColor, SetAttribute, SetForegroundColor,
        SetStyle,
    },
    ExecutableCommand,
};
use log::{debug, info};
use notion::PageResponse;
use ratatui::symbols::block;
use reqwest::blocking::ClientBuilder;
use simplelog::TermLogger;

mod html;
mod intermediary;
mod notion;

//#[derive(Debug, Deserialize)]
//pub enum Block {}

fn main() {
    let arg = std::env::args().nth(1);
    let page = match arg {
        Some(ref arg) => arg,
        _ => {
            help();
            return;
        }
    };

    let token = match std::env::var("NOTION_TOKEN") {
        Ok(token) => token,
        _ => {
            no_auth();
            return;
        }
    };

    // TODO: use logging for the errors
    TermLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    info!("Fetching page {}...", page);

    let client = ClientBuilder::new()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        .build()
        .expect("Failed to create reqwest client");

    debug!("Created reqwest client");

    let data: PageResponse = client
        .get(&format!(
            "https://api.notion.com/v1/blocks/{}/children?page_size=100",
            page
        ))
        .bearer_auth(token)
        .header("Notion-Version", "2022-06-28")
        .send()
        .expect("Failed to fetch page")
        .json()
        .expect("Failed to parse JSON");

    info!("Fetch successfully!");

    let blocks = intermediary::parse_blocks(data.results);

    //eprintln!("{:#?}", blocks);

    println!("{}", html::from_blocks(blocks, false));
}

fn no_auth() {
    let stdout = &mut stdout();

    set_bold(stdout, true).unwrap();
    print_color(stdout, Color::Red, "Error: ").unwrap();
    set_bold(stdout, false).unwrap();
    print_color(stdout, Color::DarkRed, "No auth token provided!\n\n").unwrap();
    print!("Please provide a Notion auth token in the ");
    set_bold(stdout, true).unwrap();
    print!("NOTION_TOKEN");
    set_bold(stdout, false).unwrap();
    println!(" environment variable.\n");
}

fn help() {
    let arg = std::env::args().nth(0);
    println!("Fetch a Notion page as markdown!\n");

    let stdout = &mut stdout();

    set_bold(stdout, true).unwrap();
    print_color(stdout, Color::Green, "Usage: ").unwrap();
    print_color(stdout, Color::Cyan, &arg.unwrap()).unwrap();
    set_bold(stdout, false).unwrap();
    print_color(stdout, Color::DarkCyan, " <page_id>\n").unwrap();

    stdout.execute(ResetColor).unwrap();
}

fn set_bold(stdout: &mut Stdout, bold: bool) -> Result<(), std::io::Error> {
    if bold {
        execute!(stdout, SetAttribute(Attribute::Bold))
    } else {
        execute!(stdout, SetAttribute(Attribute::NormalIntensity))
    }
}

fn print_color(stdout: &mut Stdout, color: Color, text: &str) -> Result<(), std::io::Error> {
    execute!(stdout, SetForegroundColor(color), Print(text), ResetColor)
}
