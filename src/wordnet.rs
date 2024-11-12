use thiserror::Error;
use serde::{Serialize,Deserialize,Serializer,Deserializer};
use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::path::Path;
use std::fs::File;
use std::fmt;
use serde::de::{self, Visitor, MapAccess};
use serde::ser::SerializeMap;
use indicatif::ProgressBar;

/// The Lexicon contains the whole WordNet graph
pub struct Lexicon {
    entries : HashMap<String, Entries>,
    synsets : HashMap<String, Synsets>,
    synsets_by_ili : HashMap<String, SynsetId>,
    synset_id_to_lexfile : HashMap<SynsetId, String>,
}

impl Lexicon {
    /// Create a new empty lexicon
    #[allow(dead_code)]
    pub fn new() -> Lexicon {
        Lexicon {
            entries: HashMap::new(),
            synsets: HashMap::new(),
            synsets_by_ili: HashMap::new(),
            synset_id_to_lexfile: HashMap::new(),
        }
    }

    /// Load a lexicon from a folder of YAML files
    pub fn load<P: AsRef<Path>>(folder : P) -> Result<Lexicon, WordNetYAMLIOError> {
        let mut entries : HashMap<String, Entries> = HashMap::new();
        let mut synsets = HashMap::new();
        let mut synset_id_to_lexfile = HashMap::new();
        let mut sense_id_to_lemma_pos = HashMap::new();
        let mut synsets_by_ili = HashMap::new();
        let folder_files = fs::read_dir(folder)
            .map_err(|e| WordNetYAMLIOError::Io(format!("Could not list directory: {}", e)))?;
        println!("Loading WordNet");
        let bar = ProgressBar::new(73);
        for file in folder_files {
            let file = file.map_err(|e|
                WordNetYAMLIOError::Io(format!("Could not list directory: {}", e)))?;
            let file_name = file.path().file_name().
                and_then(|x| x.to_str()).
                map(|x| x.to_string()).
                unwrap_or_else(|| "".to_string());
            if file_name.starts_with("entries-") && file_name.ends_with(".yaml") {
                let key = file_name[8..9].to_string();
                let entries2 : Entries =
                    serde_yaml::from_reader(File::open(file.path())
                        .map_err(|e| WordNetYAMLIOError::Io(format!("Error reading {} due to {}", file_name, e)))?)
                        .map_err(|e| WordNetYAMLIOError::Serde(format!("Error reading {} due to {}", file_name, e)))?;
                for (lemma, map) in entries2.0.iter() {
                    for (pos, entry) in map.iter() {
                        for sense in entry.sense.iter() {
                            sense_id_to_lemma_pos.insert(sense.id.clone(),
                                (lemma.to_string(), pos.clone()));
                        }
                    }
                }

                entries.insert(key, entries2);
            } else if file_name.ends_with(".yaml") && file_name != "frames.yaml" {
                let synsets2 : Synsets = serde_yaml::from_reader(
                    File::open(file.path())
                        .map_err(|e| WordNetYAMLIOError::Io(format!("Error reading {} due to {}", file_name, e)))?)
                        .map_err(|e| WordNetYAMLIOError::Serde(format!("Error reading {} due to {}", file_name, e)))?;
                let lexname = file_name[0..file_name.len()-5].to_string();
                for (id, value) in synsets2.0.iter() {
                    synset_id_to_lexfile.insert(id.clone(), lexname.clone());
                    synsets_by_ili.insert(value.ili.clone().unwrap_or_else(|| ILIID("".to_string())).0.clone(), id.clone());
                }
                synsets.insert(lexname, synsets2);
            }
            bar.inc(1);
        }
       bar.finish();
       Ok(Lexicon { entries, synsets, synsets_by_ili, synset_id_to_lexfile })
    }

    /// Get the lexicographer file name for a synset
    pub fn lex_name_for(&self, synset_id : &SynsetId) -> Option<String> {
        self.synset_id_to_lexfile.get(synset_id).map(|x| x.clone())
    }

    /// Get the entry data for a lemma
    pub fn entry_by_lemma(&self, lemma : &str) -> Vec<&Entry> {
        match self.entries.get(&entry_key(lemma)) {
            Some(v) => v.entry_by_lemma(lemma),
            None => Vec::new()
        }
    }

    /// Get synset data by ID
    pub fn synset_by_id(&self, synset_id : &SynsetId) -> Option<&Synset> {
        match self.lex_name_for(synset_id) {
            Some(lex_name) => {
                match self.synsets.get(&lex_name) {
                    Some(sss) => {
                        sss.0.get(synset_id)
                    },
                    None => None
                }
            },
            None => None
        }
    }

    /// Get synset by ILI
    pub fn synset_by_ili(&self, ili : &str) -> Option<(&SynsetId, &Synset)> {
        match self.synsets_by_ili.get(ili) {
            Some(ssid) => if let Some(synset) = self.synset_by_id(ssid) {
                Some((ssid, synset))
            } else {
                None
            },
            None => None
        }
    }

    /// Get the lemmas that start with a string
    pub fn lemma_by_prefix(&self, prefix: &str) -> Vec<String> {
        let prefix = prefix.to_lowercase();
        match self.entries.get(&entry_key(&prefix)) {
            Some(v) => v.lemma_by_prefix(&prefix),
            None => Vec::new()
        }
    }

    /// Get the synsets that start with an ID
    pub fn ssid_by_prefix(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        for (key, _) in self.synset_id_to_lexfile.iter() {
            let key = key.0.clone();
            if key.starts_with(&prefix) {
                result.push(key);
            }
        }
        result
    }

    /// Get the ILIs that start with a string
    pub fn ili_by_prefix(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        for (key, _) in self.synsets_by_ili.iter() {
            if key.starts_with(&prefix) {
                result.push(key.clone());
            }
        }
        result
    }
}

fn entry_key(lemma : &str) -> String {
    let key = lemma.to_lowercase().chars().next().expect("Empty lemma!");
    if key < 'a' || key > 'z' {
        '0'.to_string()
    } else {
        key.to_string()
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone,Eq,Hash,PartialOrd,Ord)]
pub struct PosKey(String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Entries(BTreeMap<String, BTreeMap<PosKey, Entry>>);

impl Entries {
    fn entry_by_lemma(&self, lemma : &str) -> Vec<&Entry> {
        self.0.get(lemma).iter().flat_map(|x| x.values()).collect()
    }

    fn lemma_by_prefix(&self, prefix: &str) -> Vec<String> {
        self.0.iter().filter(|(k, _)| k.to_lowercase().starts_with(prefix)).map(|(k, _)| k.clone()).collect()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone,Default)]
pub struct Entry {
    pub sense : Vec<Sense>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub form : Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pronunciation : Vec<Pronunciation>
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub struct Sense {
    pub id : SenseId,
    pub synset : SynsetId,
    #[serde(default)]
    pub adjposition : Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subcat: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub antonym: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub also: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub participle: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pertainym: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derivation: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub domain_topic: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_domain_topic: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub domain_region: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_domain_region: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exemplifies: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_exemplified_by: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub similar: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub other: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instrument: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub by_means_of: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub undergoer: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub state: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub uses: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub destination: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub body_part: Vec<SenseId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vehicle: Vec<SenseId>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    sent : Vec<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub struct Pronunciation {
    value : String,
    variety : Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Synsets(BTreeMap<SynsetId, Synset>);
   

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub struct Synset {
    pub definition : Vec<String>,
    #[serde(default)]
    pub example : Vec<Example>,
    pub ili : Option<ILIID>,
    wikidata : Option<String>,
    pub source : Option<String>,
    pub members : Vec<String>,
    #[serde(rename="partOfSpeech")]
    pub part_of_speech : PartOfSpeech,
    #[serde(default)]
    also : Vec<SynsetId>,
    #[serde(default)]
    attribute : Vec<SynsetId>,
    #[serde(default)]
    causes : Vec<SynsetId>,
    #[serde(default)]
    pub domain_region : Vec<SynsetId>,
    #[serde(default)]
    pub domain_topic : Vec<SynsetId>,
    #[serde(default)]
    pub exemplifies : Vec<SynsetId>,
    #[serde(default)]
    entails : Vec<SynsetId>,
    #[serde(default)]
    pub hypernym : Vec<SynsetId>,
    #[serde(default)]
    pub instance_hypernym : Vec<SynsetId>,
    #[serde(default)]
    mero_location : Vec<SynsetId>,
    #[serde(default)]
    mero_member : Vec<SynsetId>,
    #[serde(default)]
    mero_part : Vec<SynsetId>,
    #[serde(default)]
    mero_portion : Vec<SynsetId>,
    #[serde(default)]
    mero_substance : Vec<SynsetId>,
    #[serde(default)]
    meronym : Vec<SynsetId>,
    #[serde(default)]
    pub similar : Vec<SynsetId>,
    #[serde(default)]
    pub feminine : Vec<SynsetId>,
    #[serde(default)]
    pub masculine : Vec<SynsetId>,
    #[serde(default)]
    other : Vec<SynsetId>
}

#[derive(Debug, PartialEq,Clone)]
pub struct Example {
    pub text : String,
    pub source : Option<String>
}

impl Serialize for Example {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.source {
            Some(ref s) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("source", s)?;
                map.serialize_entry("text", &self.text)?;
                map.end()
            },
            None => {
                serializer.serialize_str(&self.text)
            }
        }
    }
}


impl<'de> Deserialize<'de> for Example {
    fn deserialize<D>(deserializer: D) -> Result<Example, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ExampleVisitor)
    }
}

pub struct ExampleVisitor;

impl<'de> Visitor<'de> for ExampleVisitor
{
    type Value = Example;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or map")
    }

    fn visit_str<E>(self, value: &str) -> Result<Example, E>
    where
        E: de::Error,
    {
        Ok(Example { text: value.to_string(), source: None })
    }

    fn visit_map<M>(self, mut map: M) -> Result<Example, M::Error>
    where
        M: MapAccess<'de>,
    {
        let key1 = map.next_key::<String>()?;
        let val1 = map.next_value::<String>()?;
        let key2 = map.next_key::<String>()?;
        let val2 = map.next_value::<String>()?;
        if key1 == Some("text".to_string()) && key2 == Some("source".to_string()) {
            Ok(Example { text: val1, source: Some(val2) })
        } else if key2 == Some("text".to_string()) && key1 == Some("source".to_string()) {
            Ok(Example { text: val2, source: Some(val1) })
        } else {
            panic!("Unexpected keys in example")
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub struct ILIID(String);

impl ILIID {
    #[allow(dead_code)]
    pub fn new(s : &str) -> ILIID { ILIID(s.to_string()) }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub enum PartOfSpeech { n, v, a, r, s }

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone,Eq,Hash)]
pub struct SenseId(String);

impl SenseId {
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone,Eq,Hash,PartialOrd,Ord)]
pub struct SynsetId(String);

impl SynsetId {
    pub fn new(s : &str) -> SynsetId { SynsetId(s.to_string()) }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone,Eq,Hash,PartialOrd,Ord)]
pub struct DeprecationRecord(String,String,String,String,String);

#[derive(Error,Debug)]
pub enum WordNetYAMLIOError {
    #[error("Could not load WordNet: {0}")]
    Io(String),
    #[error("Could not load WordNet: {0}")]
    Serde(String),
}

