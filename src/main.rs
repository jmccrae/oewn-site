#[macro_use] extern crate rocket;

mod hbs;
mod negotiation;
mod wordnet;

use clap::Parser;
use handlebars::Handlebars;
use negotiation::{ContentNegotiation, NegotiatedResponse, negotiated};
use rocket::config::Config as RocketConfig;
use rocket::fs::FileServer;
use rocket::response::content::{RawHtml, RawJson};
use rocket::response::Redirect;
use rocket::http::ContentType;
use once_cell::sync::OnceCell;
use wordnet::{Lexicon, SynsetId, MemberSynset};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use serde::Serialize;
use teanga::Corpus;
use teanga::disk_corpus::DiskCorpus;
use teanga::query::QueryBuilder;
use teanga::layer::TeangaData;
use teanga;

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
    handlebars : Handlebars<'a>,
    corpora : HashMap<String, DiskCorpus>,
}

static STATE: OnceCell<State> = OnceCell::new();

fn prepare_server(config : &Config) -> Result<(), String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("xml", include_str!("hbs/xml.hbs")).map_err(|e| format!("Failed to register template: {}", e))?;
    handlebars.register_template_string("rdfxml", include_str!("hbs/rdfxml.hbs")).map_err(|e| format!("Failed to register template: {}", e))?;
    handlebars.register_template_string("ttl", include_str!("hbs/ttl.hbs")).map_err(|e| format!("Failed to register template: {}", e))?;
    handlebars.register_template_string("ttl-header", include_str!("hbs/ttl-header.hbs")).map_err(|e| format!("Failed to register template: {}", e))?;
    handlebars.register_template_string("html", include_str!("hbs/html.hbs")).map_err(|e| format!("Failed to register template: {}", e))?;
    handlebars.register_template_string("sitemap", include_str!("hbs/sitemap.hbs")).map_err(|e| format!("Failed to register template: {}", e))?;
    handlebars.register_helper("lemma_escape", Box::new(hbs::lemma_escape));
    handlebars.register_helper("long_pos", Box::new(hbs::long_pos));
    let wn = if let Some(ref wn_path) = config.wn {
        Lexicon::load(wn_path)
            .map_err(|e| format!("Failed to load WordNet: {}", e))?
    } else {
        Lexicon::from_disk()
    };
    let mut corpora = HashMap::new();
    for file in vec!["raganato_ALL.yaml", "semcor.yaml"] {
    //for file in vec!["raganato_ALL.yaml", "semcor.yaml", "wngt.yaml"] {
        let name = file[..file.len()-5].to_string();
        let new_corpus = !Path::new(&format!("{}.db", name)).exists();
        let mut corpus = DiskCorpus::new(&format!("{}.db", name))
            .map_err(|e| format!("Failed to open corpus: {}", e))?;
        if new_corpus {
            eprintln!("Loading corpus {}", name);
            let file = File::open(file).map_err(|e| format!("Failed to open corpus file: {}", e))?;
            teanga::read_yaml(file, &mut corpus)
                .map_err(|e| format!("Failed to read corpus file: {}", e))?;
        }
        corpora.insert(name, corpus);
    }


    STATE.set(State { wn, handlebars, corpora }).map_err(|_| "Failed to set state".to_string())?;

    Ok(())
}

#[get("/")]
fn index_page() -> RawHtml<&'static str> {
    RawHtml(include_str!("../dist/index.html"))
}

#[get("/lemma/<lemma>")]
fn get_lemma(lemma: &str, neg: ContentNegotiation) -> Option<NegotiatedResponse> {
    let state = STATE.get().expect("State not set");
    if let Some(_) = state.wn.entry_by_lemma(lemma).iter().next() {
        Some(negotiated("lemma", lemma, include_str!("../dist/index.html"), neg))
    } else {
        None
    }
}

#[get("/id/<id>")]
fn get_id(id: &str, neg: ContentNegotiation) -> Option<NegotiatedResponse> {
    let state = STATE.get().expect("State not set");
    if id.starts_with("oewn") {
        return Some(NegotiatedResponse::Redirect(Redirect::to(format!("/id/{}", &id[5..]))));
    } else if let Some(synset) = state.wn.synset_by_id(&SynsetId::new(id)) {
        match negotiated("id", id, include_str!("../dist/index.html"), neg) {
            NegotiatedResponse::Html(RawHtml(content)) => {
                let content = String::from(content);
                let index_content = state.handlebars.render("html", &synset).expect("Failed to render template");

                let content = content.replace("<div id=\"app\"></div>",
                    &("<div id=\"app\">".to_string() + &index_content+ "</div>"));
                Some(NegotiatedResponse::HtmlDyn(RawHtml(content)))

            },
            x => Some(x)
        }
    } else {
        None
    }
}

#[get("/ili/<id>")]
fn get_ili(id: &str, neg: ContentNegotiation) -> Option<NegotiatedResponse> {
    let state = STATE.get().expect("State not set");
    if let Some(_) = state.wn.synset_by_ili(id) {
        Some(negotiated("ili", id, include_str!("../dist/index.html"), neg))
    } else {
        None
    }
}

#[get("/downloads")]
fn downloads() -> RawHtml<&'static str> {
    RawHtml(include_str!("../dist/index.html"))
}

#[get("/edit")]
fn edit_page() -> RawHtml<&'static str> {
    RawHtml(include_str!("../dist/index.html"))
}

#[get("/edit/<_ssid>")]
fn edit_page2(_ssid: &str) -> RawHtml<&'static str> {
    RawHtml(include_str!("../dist/index.html"))
}

#[get("/favicon.ico")]
fn favicon() -> (ContentType, &'static [u8]) {
    (ContentType::Icon, include_bytes!("favicon.ico"))
}

#[get("/robots.txt")]
fn robots() -> (ContentType, &'static [u8]) {
    (ContentType::Plain, include_bytes!("robots.txt"))
}

#[derive(Serialize)]
struct SiteMapData {
    synsets : Vec<SynsetId>,
}

#[get("/sitemap.xml")]
fn sitemap() -> (ContentType, String) {
    let state = STATE.get().expect("State not set");
    let synsets = state.wn.synset_ids.clone();
    let data = SiteMapData { synsets };
    (ContentType::new("application", "xml"),
        state.handlebars.render("sitemap", &data).expect("Failed to render template"))
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
struct SynsetData {
    title: String,
    lemma: String,
    definition: String,
    value: String,
}

#[get("/autocomplete_synset/<query>")]
fn autocomplete_synset(query: &str) -> RawJson<String> {
    let state = STATE.get().expect("State not set");
    let mut lemmas = state.wn.lemma_by_prefix(query);
    lemmas.sort_by(|a, b| {
        match a.to_lowercase().cmp(&b.to_lowercase()) {
            std::cmp::Ordering::Equal => a.cmp(b).reverse(),
            x => x
        }
    });
    let mut results = Vec::new();
    for lemma in lemmas {
        for ssid in state.wn.entry_by_lemma(&lemma).iter() {
            if let Some(synset) = state.wn.synset_by_id(ssid) {
                results.push(SynsetData {
                    title: format!("{} - {}", lemma, synset.definition[0]),
                    lemma: lemma.clone(),
                    definition: synset.definition[0].to_string(),
                    value: ssid.to_string()
                });
            }
        }
        if results.len() > 100 {
            break;
        }
    }
    RawJson(serde_json::to_string(&results).expect("Failed to serialize"))
}


#[derive(Serialize)]
struct JsonResponse {
    synsets: Vec<MemberSynset>,
    target_labels: HashMap<String, String>,
}

impl JsonResponse {
    fn new() -> Self {
        JsonResponse {
            synsets: Vec::new(),
            target_labels: HashMap::new()
        }
    }

    fn add_targets(&mut self, lexicon : &Lexicon) {
        for synset in self.synsets.iter() {
            for target in synset.domain_topic.iter() {
                if let Some(target_synset) = lexicon.synset_by_id(target) {
                    self.target_labels.insert(target.to_string(), 
                        target_synset.members.iter().next()
                        .map(|x| x.lemma.to_string()).unwrap_or("".to_string()));
                }
            }
        }
    }

    fn merge(&mut self, other : JsonResponse) {
        self.synsets.extend(other.synsets);
        self.target_labels.extend(other.target_labels);
    }
}

fn resolve_query<'a>(state: &'a State, index : &str, id : &str) -> Result<JsonResponse, String> {
    let mut response = JsonResponse::new();
    if index == "id" {
        let ssid = SynsetId::new(id);
        if let Some(synset) = state.wn.synset_by_id(&ssid) {
            response.synsets.push(synset);
        }
    } else if index == "lemma" {
        let entries = state.wn.entry_by_lemma(id);
        for synset in entries.iter() {
            if let Some(synset) = state.wn.synset_by_id(&synset) {
                response.synsets.push(synset);
            } else {
                return Err(format!("Failed to find synset {:?}", synset));
            }
        }
    } else if index == "ili" {
        if let Some((_, synset)) = state.wn.synset_by_ili(id) {
            response.synsets.push(synset);
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
    let hb_data = hbs::make_synsets_hb(response.synsets, index, query);
    let mut content = String::new();
    content.push_str(&state.handlebars.render("ttl-header", &hb_data).map_err(|e| format!("Failed to render template: {}", e))?);
    content.push_str(&state.handlebars.render("ttl", &hb_data).map_err(|e| format!("Failed to render template: {}", e))?);
    Ok((ContentType::new("text", "turtle"), content))
}

#[get("/rdf/<index>/<query>")]
fn rdfxml(index : &str, query : &str) -> Result<(ContentType, String) , String> {
    let state = STATE.get().expect("State not set");
    let response = resolve_query(&state, index, query)?;
    let hb_data = hbs::make_synsets_hb(response.synsets, index, query);
    Ok((ContentType::new("application", "rdf+xml"),
        state.handlebars.render("rdfxml", &hb_data).map_err(|e| format!("Failed to render template: {}", e))?))
}

#[get("/xml/<index>/<query>")]
fn xml(index : &str, query: &str) -> Result<(ContentType, String) , String> {
    let state = STATE.get().expect("State not set");
    let response = resolve_query(&state, index, query)?;
    let hb_data = hbs::make_synsets_hb(response.synsets, index, query);
    Ok((ContentType::new("application", "xml"),
        state.handlebars.render("xml", &hb_data).map_err(|e| format!("Failed to render template: {}", e))?))
}

#[get("/html_index/<ssid>")]
fn html_synset(ssid : &str) -> Result<RawHtml<String>, String> {
    let state = STATE.get().expect("State not set");
    if let Some(synset) = state.wn.synset_by_id(&SynsetId::new(ssid)) {
        let content = state.handlebars.render("html", &synset).map_err(|e| format!("Failed to render template: {}", e))?;
        Ok(RawHtml(content))
    } else {
        Err(format!("Failed to find synset {}", ssid))
    }
}

#[get("/json/ids?<id>")]
fn ids(id : Vec<&str>) -> Result<RawJson<String>, String> {
    let state = STATE.get().expect("State not set");
    let mut response = JsonResponse::new();
    for i in id {
        response.merge(resolve_query(&state, "id", i)?);
    }
    response.add_targets(&state.wn);
    Ok(RawJson(serde_json::to_string(&response).map_err(|e| format!("Failed to serialize: {}", e))?))
}


fn dump_ttl(file : &str) -> Result<(), String> {
    let mut f = std::fs::File::create(file).map_err(|e| format!("Failed to open file: {}", e))?;
    let state = STATE.get().expect("State not set");
    let data = hbs::SynsetsHB::all(&state.wn);
    state.handlebars.render_to_write("ttl-header", &data, &mut f).map_err(|e| format!("Failed to render template: {}", e))?;
    state.handlebars.render_to_write("ttl", &data, &mut f).map_err(|e| format!("Failed to render template: {}", e))?;
    Ok(())
}

#[derive(Serialize)]
struct CorpusDocument {
    text : String,
    highlights : Vec<(usize, usize)>,
}

#[get("/corpus/<id>?<offset>&<limit>")]
fn get_corpus(id : &str, offset : Option<usize>, limit : Option<usize>) -> Result<RawJson<String>, String> {
    let state = STATE.get().expect("State not set");
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(100);
    let mut results = HashMap::new();
    for (name, corpus) in state.corpora.iter() {
        let query = QueryBuilder::new()
            .value("oewn", format!("oewn-{}", id))
            .build();
        for res in corpus.search(query).skip(offset).take(limit) {
            let (_, doc) = res.map_err(|e| format!("Failed to get document: {}", e))?;
            let text = doc.text("text", corpus.get_meta())
                .map_err(|e| format!("Failed to get text: {}", e))?
                .iter().next().map(|x| x.to_string()).unwrap_or("".to_string());
            let mut highlights = Vec::new();
            for (start, end, data) in doc.indexes_data("oewn", "text", corpus.get_meta()).map_err(|e| format!("Failed to get indexes: {}", e))? {
                if let TeangaData::String(s) = data {
                    if s != id {
                        continue;
                    }
                    highlights.push((start, end));
                } else {
                    continue;
                }
            }
            results.entry(name.clone()).or_insert_with(Vec::new).push(CorpusDocument { text, highlights });
        }
    }
    Ok(RawJson(serde_json::to_string(&results).map_err(|e| format!("Failed to serialize: {}", e))?))
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
                .mount("/", routes![index_page, json, autocomplete, 
                    get_lemma, get_id, get_ili,
                    favicon, downloads, turtle,
                    rdfxml, xml, html_synset,
                    sitemap, robots,
                    autocomplete_synset, edit_page,
                    edit_page2, ids, get_corpus])
                    
        },
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }
}
