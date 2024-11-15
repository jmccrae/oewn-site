#[macro_use] extern crate rocket;

mod wordnet;
mod hbs;
mod negotiation;

use clap::Parser;
use handlebars::Handlebars;
use negotiation::{ContentNegotiation, NegotiatedResponse, negotiated};
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
    wn: Option<String>,
    #[arg(long, help = "Dump all the RDF Turtle data to this file (Server does not start)")]
    dump_ttl : Option<String>,
}

struct State<'a> {
    wn : wordnet::Lexicon,
    handlebars : Handlebars<'a>
}

static STATE: OnceCell<State> = OnceCell::new();

fn prepare_server(config : &Config) -> Result<(), String> {
    let wn = Lexicon::load(config.wn.clone().unwrap_or(".".to_string()))
        .map_err(|e| format!("Failed to load WordNet: {}", e))?;
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("xml", include_str!("hbs/xml.hbs")).map_err(|e| format!("Failed to register template: {}", e))?;
    handlebars.register_template_string("rdfxml", include_str!("hbs/rdfxml.hbs")).map_err(|e| format!("Failed to register template: {}", e))?;
    handlebars.register_template_string("ttl", include_str!("hbs/ttl.hbs")).map_err(|e| format!("Failed to register template: {}", e))?;
    handlebars.register_template_string("ttl-header", include_str!("hbs/ttl-header.hbs")).map_err(|e| format!("Failed to register template: {}", e))?;
    handlebars.register_helper("lemma_escape", Box::new(hbs::lemma_escape));
    handlebars.register_helper("long_pos", Box::new(hbs::long_pos));

    STATE.set(State { wn, handlebars }).map_err(|_| "Failed to set state".to_string())?;

    Ok(())
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../dist/index.html"))
}

#[get("/lemma/<_id>")]
fn get_lemma(_id: &str, neg: ContentNegotiation) -> NegotiatedResponse {
    negotiated("lemma", _id, include_str!("../dist/index.html"), neg)
}

#[get("/id/<_id>")]
fn get_id(_id: &str, neg: ContentNegotiation) -> NegotiatedResponse {
    negotiated("id", _id, include_str!("../dist/index.html"), neg)
}

#[get("/ili/<_id>")]
fn get_ili(_id: &str, neg: ContentNegotiation) -> NegotiatedResponse {
    negotiated("ili", _id, include_str!("../dist/index.html"), neg)
}

#[get("/downloads")]
fn downloads() -> RawHtml<&'static str> {
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

fn resolve_query<'a>(state: &'a State, index : &str, id : &str) -> Result<JsonResponse<'a>, String> {
    let mut response = JsonResponse::new();
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
    Ok(response)
}

#[get("/json/<index>/<id>")]
fn json(index: &str, id: &str) -> Result<RawJson<String>, String> {
    let state = STATE.get().expect("State not set");
    let mut response = resolve_query(&state, index, id)?;
    response.add_targets(&state.wn);
    Ok(RawJson(serde_json::to_string(&response).map_err(|e| format!("Failed to serialize: {}", e))?))
}

#[get("/ttl/<index>/<query>")]
fn turtle(index : &str, query : &str) -> Result<(ContentType, String) , String> {
    let state = STATE.get().expect("State not set");
    let response = resolve_query(&state, index, query)?;
    let hb_data = hbs::make_synsets_hb(response.synsets, &response.entries, index, query);
    let mut content = String::new();
    content.push_str(&state.handlebars.render("ttl-header", &hb_data).map_err(|e| format!("Failed to render template: {}", e))?);
    content.push_str(&state.handlebars.render("ttl", &hb_data).map_err(|e| format!("Failed to render template: {}", e))?);
    Ok((ContentType::new("text", "turtle"), content))
}

#[get("/rdf/<index>/<query>")]
fn rdfxml(index : &str, query : &str) -> Result<(ContentType, String) , String> {
    let state = STATE.get().expect("State not set");
    let response = resolve_query(&state, index, query)?;
    let hb_data = hbs::make_synsets_hb(response.synsets, &response.entries, index, query);
    Ok((ContentType::new("application", "rdf+xml"),
        state.handlebars.render("rdfxml", &hb_data).map_err(|e| format!("Failed to render template: {}", e))?))
}

#[get("/xml/<index>/<query>")]
fn xml(index : &str, query: &str) -> Result<(ContentType, String) , String> {
    let state = STATE.get().expect("State not set");
    let response = resolve_query(&state, index, query)?;
    let hb_data = hbs::make_synsets_hb(response.synsets, &response.entries, index, query);
    Ok((ContentType::new("application", "xml"),
        state.handlebars.render("xml", &hb_data).map_err(|e| format!("Failed to render template: {}", e))?))
}

fn dump_ttl(file : &str) -> Result<(), String> {
    let mut f = std::fs::File::create(file).map_err(|e| format!("Failed to open file: {}", e))?;
    let state = STATE.get().expect("State not set");
    let data = hbs::SynsetsHB::all(&state.wn);
    state.handlebars.render_to_write("ttl-header", &data, &mut f).map_err(|e| format!("Failed to render template: {}", e))?;
    state.handlebars.render_to_write("ttl", &data, &mut f).map_err(|e| format!("Failed to render template: {}", e))?;
    Ok(())
}



#[launch]
fn rocket() -> _ {
    let config = Config::parse();
    match prepare_server(&config) {
        Ok(state) => {
            if let Some(f) = &config.dump_ttl {
                dump_ttl(f).expect("Failed to dump ttl");
                std::process::exit(0);
            }
            let mut rocket_config = RocketConfig::release_default();
            rocket_config.port = config.port;
            rocket_config.workers = 30;
            rocket::custom(&rocket_config)
                .manage(state)
                .mount("/assets", FileServer::from("dist/assets"))
                .mount("/", routes![index, json, autocomplete, 
                    get_lemma, get_id, get_ili,
                    favicon, downloads, turtle,
                    rdfxml, xml])
        },
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }
}
