extern crate select;

use std;

use select::document::Document;
use select::predicate::{And, Class, Name};

use crate::cinterview::error::GenResult;
use crate::cinterview::problem::{Problem, ProblemList};
use crate::cinterview::utils::get_progress_bar;

const PROBLEM_PAGE_CNT: u32 = 4;

/// Get all urls of problems. In this layer, error will be throwed rather than being exposed.
fn get_problem_urls() -> GenResult<Vec<String>> {
    let mut urls = Vec::new();
    for page_id in 1..PROBLEM_PAGE_CNT + 1 {
        let query_url = format!(
            "https://www.nowcoder.com/ta/coding-interviews?page={}",
            page_id
        );
        let resp = reqwest::get(query_url.as_str())?;

        let result = Document::from_read(resp)?
            .find(And(Name("td"), Class("txt-left")))
            .flat_map(|x| x.children())
            .filter_map(|x| match x.attr("href") {
                Some(x) => Some(x.to_string()),
                None => None,
            })
            .collect::<Vec<String>>();
        urls.extend(result);
    }
    Ok(urls)
}

pub fn get_problems() -> ProblemList {
    let urls = get_problem_urls().expect("get_problem_urls fail! 😔😔😔");
    let mut result = ProblemList::new();
    let progress_bar = get_progress_bar(
        urls.len() as u64,
        "No local data. Fetching from internet...",
    );

    for (i, url) in urls.iter().enumerate() {
        let query_url = format!("https://www.nowcoder.com/{}", url);
        let resp = reqwest::get(query_url.as_str()).expect("reqwest::get() fail 😔😔😔");

        let document = Document::from_read(resp).expect("Document::from_read() fail 😔😔😔");

        let titles = document
            .find(And(Name("div"), Class("terminal-topic-title")))
            .map(|x| x.text().trim().to_string())
            .collect::<Vec<String>>();

        let contents = document
            .find(And(Name("div"), Class("subject-describe")))
            .map(|x| x.text().trim().to_string())
            .collect::<Vec<String>>();

        result.push(Problem {
            num: i as u32,
            name: titles.first().expect("get title fail").clone(),
            content: contents.first().expect("get contents fail").clone(),
            passed: false,
        });

        progress_bar.set_position(i as u64);
    }
    progress_bar.finish_with_message("download finish!");
    result
}
