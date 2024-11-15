use log::{debug, error, info};
use notion::PageResponse;
use owo_colors::OwoColorize;
use reqwest::{blocking::ClientBuilder, StatusCode};
use simplelog::TermLogger;

mod html;
mod intermediary;
mod notion;

//#[derive(Debug, Deserialize)]
//pub enum Block {}

fn main() {
    let executable = std::env::args().nth(0).unwrap_or("notion2html".to_string());
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

    TermLogger::init(
        if &executable == "notion2html" {
            simplelog::LevelFilter::Info
        } else {
            simplelog::LevelFilter::Debug
        },
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

    let req = client
        .get(&format!(
            "https://api.notion.com/v1/blocks/{}/children?page_size=100",
            page
        ))
        .bearer_auth(token)
        .header("Notion-Version", "2022-06-28")
        .send()
        .expect("Failed to fetch page");

    debug!("Page fetched with status {}", req.status());

    if !req.status().is_success() {
        let status = req.status();
        match status {
            StatusCode::NOT_FOUND => {
                error!(
                    "Page not found! Have you added the integration to the page you want to see?"
                );
                return;
            }
            StatusCode::UNAUTHORIZED => {
                error!("Unauthorized! Check your auth token. ");
                return;
            }
            StatusCode::TOO_MANY_REQUESTS => {
                error!("Too many requests! Please wait a bit before trying again.");
                return;
            }
            _ => {
                error!(
                    "Failed to fetch page! Errored with {} {}. ",
                    status,
                    status.canonical_reason().unwrap_or("Unknown error")
                );
                return;
            }
        }
    }

    let data: PageResponse = req.json().expect("Failed to parse JSON");

    info!("Fetch successfully!");

    debug!("Parsing blocks to intermediary...");
    let blocks = intermediary::parse_blocks(data.results);

    debug!("Converting from intermediary format to HTML...");
    println!("{}", html::from_blocks(blocks, false));
}

fn no_auth() {
    eprintln!(
        "{} {}\n",
        "Error:".bright_red().bold(),
        "No auth token provided!".red()
    );
    eprintln!(
        "Please provide a Notion auth token in the {} environment variable. ",
        "NOTION_TOKEN".bold()
    );
}

fn help() {
    eprintln!("Fetch a Notion page as markdown!\n");

    eprintln!(
        "{} {} {}",
        "Usage:".bright_green().bold(),
        "notion2html".bright_cyan(),
        "<page_id>".cyan()
    );
}
