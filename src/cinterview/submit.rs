extern crate console;

use std::collections::HashMap;
use std::env::current_dir;

use std::fmt;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::{thread, time};
use termion::color;

use rayon::prelude::*;

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
                    self.desc,
                    clean_html(&self.memo)
                )
            }
        }
    }
}

lazy_static! {
    /// Static local data root pathbuf, equals to "$HOME/.coding-interview/"
    pub static ref LANG_NUM: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("cc", 2);
        map.insert("java", 4);
        map.insert("py", 5);
        map.insert("php", 8);
        map.insert("cs", 9);
        map.insert("js", 13);
        map.insert("jsv8", 14);
        map
    };
}

pub fn submit(_test: bool, exam: bool, lang: String, nums: Vec<u32>) {
    let debug_info = !exam;

    if !debug_info {
        println!("not support yet!");
        return;
    }

    let m = read_local_code(&nums, &lang)
        .expect("read local data fail. you should under the `coding-interview` directory");
    let mut problems = read_local_problems(&PROBLEM_PATH).expect("read local problems fail");

    let progress_bar = get_progress_bar(nums.len() as u64, "submitting...");
    let submission_ids = nums
        .par_iter()
        .map(|x| {
            let problem = &problems[*x as usize];
            let code = m.get(&x).unwrap();
            progress_bar.set_message(&format!("{} {} ok!", problem.num, problem.name));
            progress_bar.inc(1);
            submit_code(
                &problem.question_id,
                &code,
                *LANG_NUM.get(&lang.as_str()).expect("invalid lang"),
            )
            .expect("submit code fail")
        })
        .collect::<Vec<u32>>();
    progress_bar.finish_with_message("submit done!");

    // TODO, use threads to speed up?
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
                5 => problems[n as usize].passed = true,
                _ => {}
            };
            let problem = &problems[nums[n as usize] as usize];
            print_submit_resp(problem.num, &problem.name, &resp);
            break;
        }
    }

    update_problems(problems).expect("update problem fail");
}

fn print_submit_resp(num: u32, name: &str, resp: &SubmissionStatusResp) {
    println!(
        "{}---------------[{}_{}]---------------\n{}\n",
        color::Fg(color::White),
        num,
        name,
        resp
    );
}

fn read_local_code(nums: &Vec<u32>, suffix: &String) -> GenResult<HashMap<u32, String>> {
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
