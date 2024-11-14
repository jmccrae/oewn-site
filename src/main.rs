#[macro_use] extern crate rocket;

mod wordnet;

use clap::Parser;
use rocket::config::Config as RocketConfig;
use rocket::fs::FileServer;
use rocket::response::content::{RawHtml, RawJson};
use rocket::http::ContentType;
use once_cell::sync::OnceCell;
use wordnet::{Lexicon, SynsetId, Synset, Entry};
use std::collections::HashMap;
use serde::Serialize;


#[derive(Parser,Debug)]
#[command(name = "English WordNet Interface")]
#[command(version = "1.0")]
#[command(author = "John P. McCrae <john@mccr.ae>")]
struct Config {
    #[arg(short, long, default_value = "8000", help = "The port to start the server on")]
    port: u16,
    #[arg(long, help = "Reload the wordnet from the given folder")]
    wn: Option<String>
}

struct State {
    wn : wordnet::Lexicon
}

static STATE: OnceCell<State> = OnceCell::new();

fn prepare_server(config : &Config) -> Result<(), String> {
    let wn = Lexicon::load(config.wn.clone().unwrap_or(".".to_string()))
        .map_err(|e| format!("Failed to load WordNet: {}", e))?;

    STATE.set(State { wn }).map_err(|_| "Failed to set state".to_string())?;

    Ok(())
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../dist/index.html"))
}

#[get("/lemma/<_id>")]
fn get_lemma(_id: &str) -> RawHtml<&'static str> {
    RawHtml(include_str!("../dist/index.html"))
}

#[get("/id/<_id>")]
fn get_id(_id: &str) -> RawHtml<&'static str> {
    RawHtml(include_str!("../dist/index.html"))
}

#[get("/ili/<_id>")]
fn get_ili(_id: &str) -> RawHtml<&'static str> {
    RawHtml(include_str!("../dist/index.html"))
}

#[get("/favicon.ico")]
fn favicon() -> (ContentType, &'static [u8]) {
    (ContentType::Icon, include_bytes!("favicon.ico"))
}

#[get("/autocomplete/<index>/<query>")]
fn autocomplete(index : &str, query: &str) -> RawJson<String> {
    let state = STATE.get().expect("State not set");
    let mut results = if index == "lemma" {
        state.wn.lemma_by_prefix(query)
    } else if index == "ili" {
        state.wn.ili_by_prefix(query)
    } else if index == "id" {
        state.wn.ssid_by_prefix(query)
    } else {
        Vec::new()
    };
    results.sort_by(|a, b| {
        match a.to_lowercase().cmp(&b.to_lowercase()) {
            std::cmp::Ordering::Equal => a.cmp(b).reverse(),
            x => x
        }
    });
    let results = results.iter().take(100).collect::<Vec<_>>();
    RawJson(serde_json::to_string(&results).expect("Failed to serialize"))
}

#[derive(Serialize)]
struct JsonResponse<'a> {
    synsets: Vec<&'a Synset>,
    entries: HashMap<String, Vec<&'a Entry>>,
    target_labels: HashMap<String, String>,
}

impl<'a> JsonResponse<'a> {
    fn new() -> Self {
        JsonResponse {
            synsets: Vec::new(),
            entries: HashMap::new(),
            target_labels: HashMap::new()
        }
    }

    fn add_targets(&mut self, lexicon : &Lexicon) {
        for synset in self.synsets.iter() {
            for target in synset.domain_topic.iter() {
                if let Some(target_synset) = lexicon.synset_by_id(target) {
                    self.target_labels.insert(target.to_string(), 
                        target_synset.members.iter().next()
                        .map(|x| x.to_string()).unwrap_or("".to_string()));
                }
            }
        }
    }
}

#[get("/json/<index>/<id>")]
fn json(index: &str, id: &str) -> Result<RawJson<String>, String> {
    let state = STATE.get().expect("State not set");
    let mut response = JsonResponse::new();
    eprintln!("{} {}", index, id);
    if index == "id" {
        let ssid = SynsetId::new(id);
        if let Some(synset) = state.wn.synset_by_id(&ssid) {
            response.synsets.push(synset);
            for member in synset.members.iter() {
                for entry in state.wn.entry_by_lemma(&member) {
                    response.entries.entry(member.to_string()).or_insert_with(|| Vec::new()).push(entry);
                }
            }
        }
    } else if index == "lemma" {
        let entries = state.wn.entry_by_lemma(id);
        for entry in entries.iter() {
            for sense in entry.sense.iter() {
                if let Some(synset) = state.wn.synset_by_id(&sense.synset) {
                    response.synsets.push(synset);
                    for member in synset.members.iter() {
                        if response.entries.contains_key(member) {
                            continue;
                        }
                        for entry in state.wn.entry_by_lemma(&member) {
                            response.entries.entry(member.to_string()).or_insert_with(|| Vec::new()).push(entry);
                        }
                    }
                } else {
                    return Err(format!("Failed to find synset {:?}", sense.synset));
                }
            }
        }
        response.entries.insert(id.to_string(), entries);
    } else if index == "ili" {
        if let Some((_, synset)) = state.wn.synset_by_ili(id) {
            response.synsets.push(synset);
            for member in synset.members.iter() {
                for entry in state.wn.entry_by_lemma(&member) {
                    response.entries.entry(member.to_string()).or_insert_with(|| Vec::new()).push(entry);
                }
            }
        }
    } else {
        return Err("Invalid index".to_string())
    }
    response.add_targets(&state.wn);
    Ok(RawJson(serde_json::to_string(&response).map_err(|e| format!("Failed to serialize: {}", e))?))
}

#[launch]
fn rocket() -> _ {
    let config = Config::parse();
    match prepare_server(&config) {
        Ok(state) => {
            let mut rocket_config = RocketConfig::release_default();
            rocket_config.port = config.port;
            rocket_config.workers = 30;
            rocket::custom(&rocket_config)
                .manage(state)
                .mount("/assets", FileServer::from("dist/assets"))
                .mount("/", routes![index, json, autocomplete, 
                    get_lemma, get_id, get_ili,
                    favicon])
        },
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }
}
