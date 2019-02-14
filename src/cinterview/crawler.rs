extern crate select;

use std;

use select::document::Document;
use select::predicate::{Name, Class, And};

use crate::cinterview::problem::{ProblemList, Problem};
use crate::cinterview::error::{GenResult};

const PROBLEM_PAGE_CNT: u32 = 4;

/// Get all urls of problems. In this layer, error will be throwed rather than being exposed.
fn get_problem_urls() -> GenResult<Vec<String>> {
    let mut urls = Vec::new();
    for page_id in 1..PROBLEM_PAGE_CNT+1 {
        let query_url = format!("https://www.nowcoder.com/ta/coding-interviews?page={}", page_id);
        let resp = reqwest::get(query_url.as_str())?;

        let result = Document::from_read(resp)?
            .find(And(Name("td"), Class("txt-left")))
            .flat_map(|x| x.children())
            .filter_map(
                |x| match x.attr("href") {
                    Some(x) => Some(x.to_string()),
                    None => None
            })
            .collect::<Vec<String>>();
        urls.extend(result);
    }
    Ok(urls)
}

pub fn get_problem_details() -> ProblemList {
    let urls = get_problem_urls().expect("error: [get_problem_urls] fail! 😔😔😔");
    let mut result = ProblemList::new();

    for (i, url) in urls.iter().enumerate() {
        let query_url= format!("https://www.nowcoder.com/{}", url);
        let resp = reqwest::get(query_url.as_str())
            .expect(format!("error: reqwest::get(), url: {} 😔😔😔", query_url).as_str());
        
        let document = Document::from_read(resp)
            .expect("error: fail to get resp from result of reqwest::get() 😔😔😔");
        let titles = document.find(And(Name("div"), Class("terminal-topic-title")))
            .map(|x| x.text().trim().to_string())
            .collect::<Vec<String>>();

        let contents = document.find(And(Name("div"), Class("subject-describe")))
            .map(|x| x.text().trim().to_string())
            .collect::<Vec<String>>();

        result.push(Problem{
            num: i as u32,
            name: titles.first().expect("❗️❗️❗️error: [get_problem_details] can't unwrap title").clone(),
            content: contents.first().expect("❗️❗️❗️error: [get_problem_details] can't unwrap content").clone(),
            passed: false
        })
    }
    result
}

#[test]
fn test() {
    
}