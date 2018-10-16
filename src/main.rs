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

use rand::random;

lazy_static! {
    static ref SAMPLE_TYPES: Vec<&'static str> = vec![
        "String",
        "Int",
        "Bool",
        "List"
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
}

static MAX_SAMPLE_TYPES: usize = 10;

#[derive(Serialize, Deserialize, Debug)]
struct SearchResultRepo {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResultFn {
    name: String,
    args: Vec<String>,
    ret: String,
    desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResult {
    repo: SearchResultRepo,
    res: SearchResultFn,
}

//fn gen_rand_search_result() -> String {
//    let num_types: usize = random() % MAX_SAMPLE_TYPES;
//    serde_json::to_string(&point).unwrap()
//}

#[get("/search/<s>")]
fn search(s: String) -> String {
    serde_json::to_string::<Vec<SearchResult>>(
        vec![SearchResult{
            repo: SearchResultRepo{
                name: String::from("test"),
                url: String::from("test"),
            },
            res: SearchResultFn{
                name: String::from("hello"),
                args: vec![String::from("String"), String::from("Int")],
                ret: String::from("Bool"),
                desc: String::from("this sure is a description"),
            },
        }].as_ref()
    ).unwrap()
}


fn main() {
    rocket::ignite().mount("/", routes![search]).launch();
}