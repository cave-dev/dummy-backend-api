#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rocket_cors;

use rand::random;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

lazy_static! {
    static ref SAMPLE_TYPES: Vec<&'static str> = vec![
        "String",
        "Int",
        "Bool",
        "List",
        "Time"
    ];
    static ref SAMPLE_SITES: Vec<&'static str> = vec![
        "https://github.com/",
        "https://bitbucket.com/",
        "https://gitlab.com/"
    ];
    static ref SAMPLE_USERS: Vec<&'static str> = vec![
        "bubby",
        "ffrancis",
        "captain_oblivious",
        "xXx_fortnitememes_xXx",
        "koopa",
        "derpington",
        "pecan"
    ];
    static ref SAMPLE_REPO_NAMES: Vec<&'static str> = vec![
        "awesomefifo",
        "epic_javascript_library",
        "NoJs",
        "pls",
        "AlT-cAsE-oNlY-cAsE",
        "derp"
    ];
    static ref SAMPLE_FUNCTION_NAMES: Vec<&'static str> = vec![
        "test_fn",
        "bogosort",
        "derpyfunc",
        "dothings",
        "FUnCtiOn_stUUUF"
    ];
}

static MAX_SAMPLE_TYPES: usize = 10;
static MAX_NUMBER_RESULTS: usize = 10;

#[derive(Serialize, Deserialize, Debug)]
struct SearchResultRepo {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResultFn {
    name: String,
    desc: String,
    args: Vec<String>,
    ret: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResult {
    repo: SearchResultRepo,
    res: SearchResultFn,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResultWrapper {
    data: Vec<SearchResult>,
}

fn gen_repo_url() -> (String, String) {
    let mut res = String::new();
    let url_index: usize = random::<usize>() % SAMPLE_SITES.len();
    res.push_str(SAMPLE_SITES[url_index]);
    let user_index: usize = random::<usize>() % SAMPLE_USERS.len();
    res.push_str(SAMPLE_USERS[user_index]);
    res.push('/');
    let repo_index: usize = random::<usize>() % SAMPLE_REPO_NAMES.len();
    res.push_str(SAMPLE_REPO_NAMES[repo_index]);
    (String::from(SAMPLE_REPO_NAMES[repo_index]), res)
}

fn gen_rand_search_result() -> String {
    let mut results: Vec<SearchResult> = Vec::new();
    let num_results: usize = random::<usize>() % MAX_NUMBER_RESULTS;
    for _ in 0..num_results {
        let mut types: Vec<String> = Vec::new();
        let num_types: usize = random::<usize>() % SAMPLE_SITES.len();
        for _ in 0..num_types {
            let type_num: usize = random::<usize>() % SAMPLE_TYPES.len();
            types.push(String::from(SAMPLE_TYPES[type_num]));
        }
        let type_index: usize = random::<usize>() % SAMPLE_TYPES.len();
        let fn_index: usize = random::<usize>() % SAMPLE_FUNCTION_NAMES.len();
        let res_fn = SearchResultFn{
            name: String::from(SAMPLE_FUNCTION_NAMES[fn_index]),
            desc: String::from(SAMPLE_FUNCTION_NAMES[fn_index]),
            args: types,
            ret: String::from(SAMPLE_TYPES[type_index]),
        };
        let (repo_name, repo_url) = gen_repo_url();
        let res_repo = SearchResultRepo{
            name: repo_name,
            url: repo_url,
        };
        results.push(SearchResult{
            repo: res_repo,
            res: res_fn,
        })
    }
    let res_wrapper = SearchResultWrapper{
        data: results,
    };
    serde_json::to_string(&res_wrapper).unwrap()
}

#[get("/search/<s>")]
fn search(s: String) -> String {
    gen_rand_search_result()
}


fn main() {
    // You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .mount("/", routes![search])
        .attach(options)
        .launch();
}