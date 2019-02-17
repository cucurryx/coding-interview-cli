extern crate select;
extern crate regex;

use std;
use std::collections::HashMap;
use std::str;

use regex::Regex;

use select::document::Document;
use select::predicate::{And, Class, Name};

use crate::cinterview::error::GenResult;
use crate::cinterview::problem::{Problem, ProblemList};
use crate::cinterview::utils::get_progress_bar;

const PROBLEM_PAGE_CNT: u32 = 4;

/// Static hash map that contains the suffix of each programming languages
lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("javaTpl", "java");
        map.insert("cTpl", "cc");
        map.insert("pythonTpl", "py");
        map.insert("cSharpTpl", "cs");
        map.insert("jsTpl", "js");
        map.insert("phpTpl", "php");
        map.insert("jsV8Tpl", "jsv8");
        map
    };
}

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
        let mut resp = reqwest::get(query_url.as_str()).expect("reqwest::get() fail 😔");
        let body = resp.text().expect("get resp body fail");
        let document = Document::from(body.as_str());

        let titles = document
            .find(And(Name("div"), Class("terminal-topic-title")))
            .map(|x| x.text().trim().to_string())
            .collect::<Vec<String>>();

        let contents = document
            .find(And(Name("div"), Class("subject-describe")))
            .map(|x| x.text().trim().to_string())
            .collect::<Vec<String>>();

        let templates = document
            .find(Name("textarea"))
            .map(|x| {
                (
                    to_suffix(x.attr("id").expect("get id fail")),
                    x.text().to_string(),
                )
            })
            .into_iter()
            .collect::<HashMap<String, String>>();

        let re = Regex::new("[0-9]+").expect("regex compile error");
        let line = body
            .as_str()
            .split("\n")
            .map(|x| x.to_string())
            .find(|x| x.starts_with("questionId"))
            .expect("get question id fail");
        let match_res = re.find(line.as_str()).expect("get question id fail");
        let question_id = line[match_res.start()..match_res.end()].to_string();

        result.push(Problem {
            num: i as u32,
            question_id: question_id,
            name: titles.first().expect("get title fail").clone(),
            content: contents.first().expect("get contents fail").clone(),
            passed: false,
            templates: templates,
        });

        progress_bar.set_position(i as u64);
    }
    progress_bar.finish_with_message("download finish!");
    result
}

fn to_suffix(key: &str) -> String {
    PRIVILEGES.get(key).unwrap_or(&"unknown").to_string()
}
