mod models;
mod parser;

use models::{JeopardyQuestion, Round};
use parser::parse_questions;
use scraper::Selector;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://j-archive.com/showgame.php?game_id=7515";
    let raw_html = reqwest::get(url).await?.text().await?;
    let document = scraper::Html::parse_document(&raw_html);

    let jr_selector = Selector::parse("#jeopardy_round").unwrap();
    let djr_selector = Selector::parse("#double_jeopardy_round").unwrap();
    let fjr_selector = Selector::parse("#final_jeopardy_round").unwrap();

    let jr_table = document.select(&jr_selector).next().unwrap();
    let djr_table = document.select(&djr_selector).next().unwrap();
    let fjr_table = document.select(&fjr_selector).next().unwrap();

    let jr_questions: Vec<JeopardyQuestion> = parse_questions(&jr_table, Round::Jeopardy);
    let djr_questions: Vec<JeopardyQuestion> = parse_questions(&djr_table, Round::DoubleJeopardy);
    let fjr_question = parse_questions(&fjr_table, Round::FinalJeopardy);

    for question in jr_questions {
        println!("{:?}", question);
    }

    for question in djr_questions {
        println!("{:?}", question);
    }

    println!("fjr = {:?}", fjr_question);

    Ok(())
}
