extern crate console;

use std::collections::HashMap;
use std::env::current_dir;

use std::fmt;

use std::fs::read_to_string;
use std::path::PathBuf;
use std::{thread, time};

use termion::color;

use console::{style};

use crate::cinterview::config::*;
use crate::cinterview::error::*;
use crate::cinterview::problem::*;
use crate::cinterview::utils::*;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct SubmitResp {
    msg: String,
    code: u32,
    submissionId: u32,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct SubmissionStatusResp {
    testcaseresults: Option<String>,
    seconds: Option<u32>,
    memory: Option<u32>,
    code: u32,
    memo: String,
    place: u32,
    isComplete: bool,
    status: u32,
    desc: String,
}

impl fmt::Display for SubmissionStatusResp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            5 => {
                write!(f, "{}", color::Fg(color::Green))?;
                write!(
                    f,
                    "[PASS]\n memory used:\t{}k\n time used:\t{}ms\n",
                    self.memory.expect("get memory fail"),
                    self.seconds.expect("get second fail")
                )
            }
            _ => {
                write!(f, "{}", color::Fg(color::Red))?;
                write!(
                    f,
                    "[FAIL]\n\tresult:\t{}\n\tdetail:\n⬇⬇⬇⬇⬇ \n{}\n⬆⬆⬆⬆⬆\n",
                    self.desc, clean_html(&self.memo)
                )
            }
        }
    }
}

pub fn submit(_test: bool, exam: bool, nums: Vec<u32>) {
    let debug_info = !exam;

    if !debug_info {
        println!("not support yet!");
        return;
    }

    let m = read_local_code(&nums, "cc".to_string())
        .expect("read local data fail. you should under the `coding-interview` directory");
    let mut problems = read_local_problems(&PROBLEM_PATH).expect("read local problems fail");
    let submission_ids = nums
        .iter()
        .enumerate()
        .map(|(n, x)| {
            let problem = &problems[*x as usize];
            let code = m.get(x).unwrap();
            let style_str = format!("[{}/{}]", n, nums.len());
            println!(
                "{}\t{} {} submitting...",
                style(&style_str).bold().dim(),
                problem.num,
                problem.name
            );
            submit_code(&problem.question_id, &code, 2).expect("submit code fail")
        })
        .collect::<Vec<u32>>();

    let half_second = time::Duration::from_millis(500);
    for (n, x) in submission_ids.iter().enumerate() {
        let spinner = get_progress_spinner(100 as u64, &"waiting for result...");
        let _ = thread::spawn(move || {
            spinner.tick();
        });
        loop {
            thread::sleep(half_second);
            let resp = query_submission_status(*x).expect("query submission status fail");
            match resp.status {
                0 => continue,
                5 => {
                    problems[n as usize].passed = true;
                }
                _ => {}
            };
            let problem = &problems[n as usize];
            println!(
                "{}---------------------[{}_{}]---------------------",
                color::Fg(color::White),
                problem.num,
                problem.name
            );
            println!("{}\n", resp);
            break;
        }
    }

    update_problems(problems).expect("update problem fail");
}

fn read_local_code(nums: &Vec<u32>, suffix: String) -> GenResult<HashMap<u32, String>> {
    let code_root = code_root_dir()?;
    let to_submit = read_local_problems(&PROBLEM_PATH)?
        .into_iter()
        .filter(|x| nums.clone().into_iter().find(|n| n == &x.num).is_some())
        .collect::<ProblemList>();

    let mut result = HashMap::new();
    for x in to_submit {
        let filename = format!("{}_{}", x.num, x.name);
        let path = code_root
            .join(&filename)
            .join(&filename)
            .with_extension(&suffix);
        result.insert(x.num, read_to_string(path)?);
    }

    Ok(result)
}

pub fn code_root_dir() -> GenResult<PathBuf> {
    current_dir()?
        .ancestors()
        .skip_while(|x| !x.ends_with("coding-interview"))
        .next()
        .and_then(|x| Some(x.to_path_buf()))
        .ok_or(Box::new(CodeRootError {}))
}

fn submit_code(question_id: &String, code: &String, lang: u32) -> GenResult<u32> {
    let params = [
        ("questionId", question_id),
        ("content", code),
        ("language", &lang.to_string()),
    ];
    let client = reqwest::Client::new();
    let res: SubmitResp = client
        .post("https://www.nowcoder.com/submit_cd?")
        .form(&params)
        .send()?
        .json()?;
    Ok(res.submissionId)
}

fn query_submission_status(submission_id: u32) -> GenResult<SubmissionStatusResp> {
    let url = format!(
        "https://www.nowcoder.com/status?submissionId={}",
        submission_id
    );
    let resp: SubmissionStatusResp = reqwest::get(&url)?.json()?;
    Ok(resp)
}

fn clean_html(desc: &String) -> String {
    desc.split("<br/>").collect::<Vec<&str>>().join("")
}