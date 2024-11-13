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

                let entries2 = entries2.0.into_iter().map(|(lemma, map)| {
                    (lemma.clone(), map.into_iter().map(|(pos, entry)| {
                        (pos.clone(), Entry {
                            poskey: Some(pos.clone()),
                            ..entry
                        })
                    }).collect::<BTreeMap<_,_>>())
                }).collect::<BTreeMap<_,_>>();

                entries.insert(key, Entries(entries2));
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
                let synsets2 = synsets2.0.into_iter().map(|(ssid, synset)| {
                    (ssid.clone(), Synset {
                        id: Some(ssid.clone()),
                        lexname: Some(lexname.clone()),
                        ..synset
                    })
                }).collect::<BTreeMap<_,_>>();
                synsets.insert(lexname, Synsets(synsets2));
            }
            bar.inc(1);
        }
       bar.finish();
       let mut lexicon = Lexicon { entries, synsets, synsets_by_ili, synset_id_to_lexfile };
       lexicon.add_reverse_links();
       Ok(lexicon)
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

    fn synset_by_id_mut(&mut self, synset_id : &SynsetId) -> Option<&mut Synset> {
        match self.lex_name_for(synset_id) {
            Some(lex_name) => {
                match self.synsets.get_mut(&lex_name) {
                    Some(sss) => {
                        sss.0.get_mut(synset_id)
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

    /// Augment the lexicon with reverse and sense links
    pub fn add_reverse_links(&mut self) {
        macro_rules! add_reverse_links {
            ($rel:ident, $inv:ident) => {
                let mut elems = Vec::new();
                for synsets in self.synsets.values() {
                    for synset in synsets.0.values() {
                        for hyp in synset.$rel.iter() {
                            elems.push((synset.id.clone().unwrap(), hyp.clone()));
                        }
                    }
                }
                for (child, parent) in elems {
                    if let Some(parent_synset) = self.synset_by_id_mut(&parent) {
                        parent_synset.$inv.push(child.clone());
                    }
                }
            }
        }

        add_reverse_links!(hypernym, hyponym);
        add_reverse_links!(instance_hypernym, instance_hyponym);
        add_reverse_links!(mero_member, holo_member);
        add_reverse_links!(mero_part, holo_part);
        add_reverse_links!(mero_substance, holo_substance);
        add_reverse_links!(causes, is_caused_by);
        add_reverse_links!(exemplifies, is_exemplified_by);
        add_reverse_links!(entails, is_entailed_by);

        let mut sense_ids = HashMap::new();
        for entries in self.entries.values() {
            for (lemma, by_pos) in entries.0.iter() {
                for entry in by_pos.values() {
                    for sense in entry.sense.iter() {
                        sense_ids.insert(sense.id.clone(), (lemma.clone(), sense.synset.clone()));
                    }
                }
            }
        }


        macro_rules! add_sense_links {
            ($rel:ident, $inv:ident) => {
                let mut elems = Vec::new();
                for entries in self.entries.values() {
                    for (lemma, by_pos) in entries.0.iter() {
                        for entry in by_pos.values() {
                            for sense in entry.sense.iter() {
                                for target in sense.$rel.iter() {
                                    if let Some((target_lemma, synset)) = sense_ids.get(target) {
                                        elems.push((
                                            sense.synset.clone(),
                                            synset.clone(),
                                            lemma.clone(),
                                            target_lemma.clone(),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }

                for (source_synset, target_synset, source_lemma, target_lemma) in elems {
                    if let Some(synset) = self.synset_by_id_mut(&source_synset) {
                        synset.$rel.push(SenseRelation {
                            target_synset: target_synset.clone(),
                            source_lemma: source_lemma.clone(),
                            target_lemma: target_lemma.clone()
                        });
                    }
                    if let Some(synset) = self.synset_by_id_mut(&target_synset) {
                        synset.$inv.push(SenseRelation {
                            target_synset: source_synset,
                            source_lemma: target_lemma,
                            target_lemma: source_lemma
                        });
                    }
                }
            }
        }
        add_sense_links!(antonym, antonym);
        add_sense_links!(participle, is_participle_of);
        add_sense_links!(pertainym, is_pertainym_of);
        add_sense_links!(derivation, derivation);
        //add_sense_links!(exemplifies_sense, is_exemplified_by_sense);
        {
            let mut elems = Vec::new();
            for entries in self.entries.values() {
                for (lemma, by_pos) in entries.0.iter() {
                    for entry in by_pos.values() {
                        for sense in entry.sense.iter() {
                            for target in sense.exemplifies.iter() {
                                if let Some((target_lemma, synset)) = sense_ids.get(target) {
                                    elems.push((
                                            sense.synset.clone(),
                                            synset.clone(),
                                            lemma.clone(),
                                            target_lemma.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }

            for (source_synset, target_synset, source_lemma, target_lemma) in elems {
                if let Some(synset) = self.synset_by_id_mut(&source_synset) {
                    synset.exemplifies_sense.push(SenseRelation {
                        target_synset: target_synset.clone(),
                        source_lemma: source_lemma.clone(),
                        target_lemma: target_lemma.clone()
                    });
                }
                if let Some(synset) = self.synset_by_id_mut(&target_synset) {
                    synset.is_exemplified_by_sense.push(SenseRelation {
                        target_synset: source_synset,
                        source_lemma: target_lemma,
                        target_lemma: source_lemma
                    });
                }
            }
        } 
        add_sense_links!(agent, is_agent_of);
        add_sense_links!(material, is_material_of);
        add_sense_links!(event, is_event_of);
        add_sense_links!(instrument, is_instrument_of);
        add_sense_links!(location, is_location_of);
        add_sense_links!(by_means_of, is_by_means_of);
        add_sense_links!(undergoer, is_undergoer_of);
        add_sense_links!(property, is_property_of);
        add_sense_links!(result, is_result_of);
        add_sense_links!(state, is_state_of);
        add_sense_links!(uses, is_used_by);
        add_sense_links!(destination, is_destination_of);
        add_sense_links!(body_part, is_body_part_of);
        add_sense_links!(vehicle, is_vehicle_of);
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
    pronunciation : Vec<Pronunciation>,
    #[serde(default)]
    pub poskey : Option<PosKey>
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
    // not found in serialized data
    #[serde(default)]
    pub id : Option<SynsetId>,
    // not found in serialized data
    #[serde(default)]
    pub lexname: Option<String>,
    pub definition : Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub example : Vec<Example>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ili : Option<ILIID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wikidata : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source : Option<String>,
    #[serde(default)]
    pub members : Vec<String>,
    #[serde(rename="partOfSpeech")]
    pub part_of_speech : PartOfSpeech,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    also : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attribute : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    causes : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub domain_region : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub domain_topic : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exemplifies : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    entails : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hypernym : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instance_hypernym : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    mero_member : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    mero_part : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    mero_substance : Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub similar : Vec<SynsetId>,
    /// Extra values that need to be inferred
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hyponym: Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_caused_by: Vec<SynsetId>,
    #[serde(skip)]
    pub has_domain_region: Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_domain_topic: Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_exemplified_by: Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_entailed_by: Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instance_hyponym: Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub holo_member: Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub holo_part: Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub holo_substance: Vec<SynsetId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub antonym: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub participle: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_participle_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pertainym: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_pertainym_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub derivation: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exemplifies_sense: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_exemplified_by_sense: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_agent_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_material_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_event_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instrument: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_instrument_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_location_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub by_means_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_by_means_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub undergoer: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_undergoer_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_property_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_result_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub state: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_state_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub uses: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_used_by: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub destination: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_destination_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub body_part: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_body_part_of: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vehicle: Vec<SenseRelation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_vehicle_of: Vec<SenseRelation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub struct SenseRelation {
    pub target_synset: SynsetId,
    pub source_lemma: String,
    pub target_lemma: String
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

