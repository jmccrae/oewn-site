/// Handlebars templates
///

use crate::wordnet::{Entry, Synset, SynsetId, ILIID, PartOfSpeech, Pronunciation};
use handlebars::{Handlebars, Helper, HelperResult, Output, RenderContext, Context};
use std::collections::HashMap;
use serde::{Serialize,Deserialize};

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct SynsetsHB {
    synsets : Vec<HBSynset>,
    entries : HashMap<String,HashMap<String, Vec<HBSynset>>>,
    index : String,
    name : String,
    license : &'static str,
    site : &'static str
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct HBSynset {
    pub definition : String,
    pub examples: Vec<String>,
    pub lemmas : Vec<Sense>,
    pub id : SynsetId,
    pub ili : ILIID,
    pub pos : PartOfSpeech,
    pub subject : String,
    pub relations : Vec<Relation>,
}

impl HBSynset {
    pub fn from(synset : &Synset, entries: &HashMap<String, Vec<&Entry>>) -> HBSynset { 
        let mut lemmas = Vec::new();
        for member in synset.members.iter() {
            if let Some(entries) = entries.get(member) {
                for entry in entries.iter() {
                    for (idx, sense) in entry.sense.iter().enumerate() {
                        if Some(&sense.synset) != synset.id.as_ref() {
                            continue;
                        }
                        lemmas.push(Sense::from(member, entry, idx));
                    }
                }
            }
        }
        let mut relations = Vec::new();
        macro_rules! add_rel {
            ($rel:ident) => {
                for r in synset.$rel.iter() {
                    relations.push(Relation {
                        src_word: None,
                        trg_word: None,
                        rel_type: stringify!($rel).to_string(),
                        target: r.to_string()
                    });
                }
            }
        }
        add_rel!(also);
        add_rel!(attribute);
        add_rel!(causes);
        add_rel!(domain_region);
        add_rel!(domain_topic);
        add_rel!(exemplifies);
        add_rel!(entails);
        add_rel!(hypernym);
        add_rel!(instance_hypernym);
        add_rel!(mero_member);
        add_rel!(mero_part);
        add_rel!(mero_substance);
        add_rel!(similar);
        add_rel!(hyponym);
        add_rel!(is_caused_by);
        add_rel!(has_domain_region);
        add_rel!(has_domain_topic);
        add_rel!(is_exemplified_by);
        add_rel!(is_entailed_by);
        add_rel!(instance_hyponym);
        add_rel!(holo_member);
        add_rel!(holo_part);
        add_rel!(holo_substance);
        macro_rules! add_srel {
            ($rel:ident, $alias:ident) => {
                for r in synset.$rel.iter() {
                    relations.push(Relation {
                        src_word: Some(r.source_lemma.clone()),
                        trg_word: Some(r.target_lemma.clone()),
                        rel_type: stringify!($alias).to_string(),
                        target: r.target_synset.to_string()
                    });
                }
            }
        }
        add_srel!(antonym, antonym);
        add_srel!(participle, participle);
        add_srel!(is_participle_of, is_participle_of);
        add_srel!(pertainym, pertainym);
        add_srel!(is_pertainym_of, is_pertainym_of);
        add_srel!(derivation, derivation);
        add_srel!(exemplifies_sense, exemplifies);
        add_srel!(agent, agent);
        add_srel!(is_agent_of, involved_agent);
        add_srel!(instrument, instrument);
        add_srel!(is_instrument_of, involved_instrument);
        add_srel!(location, location);
        add_srel!(is_location_of, involved_location);
        HBSynset {
            definition: synset.definition.iter().next().map(|x| x.to_string()).unwrap_or("".to_string()),
            examples: synset.example.iter().map(|x| x.text.to_string()).collect(),
            lemmas,
            id: synset.id.clone().unwrap(),
            ili: synset.ili.clone().unwrap(),
            pos: synset.part_of_speech.clone(),
            subject: synset.lexname.clone().unwrap_or("".to_string()),
            relations
        }
    }
}


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Sense {
    pub lemma : String,
    pub language : String,
    pub forms : Vec<String>,
    pub sense_key : Option<String>,
    pub subcats : Vec<String>,
    pub pronunciations : Vec<Pronunciation>,
}

impl Sense {
    pub fn from(member : &str, entry: &Entry, idx: usize) -> Sense {

        Sense {
            lemma: member.to_string(),
            language: "en".to_string(),
            forms: entry.form.clone(),
            sense_key: Some(entry.sense[idx].id.to_string()),
            subcats: entry.sense[idx].subcat.clone(),
            pronunciations: entry.pronunciation.clone(),
        }
    }
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Relation {
    pub src_word : Option<String>,
    pub trg_word : Option<String>,
    pub rel_type : String,
    pub target : String
}

pub fn make_synsets_hb(synset_data : Vec<&Synset>, entry_data : &HashMap<String, Vec<&Entry>>,
                    index : &str, name : &str) -> SynsetsHB {
    let mut entries = HashMap::new();
    let mut synsets = Vec::new();
    for synset in synset_data.iter() {
        for lemma in synset.members.iter() {
            let s2 = HBSynset::from(synset, &entry_data);
            entries.entry("en".to_string())
                .or_insert_with(|| HashMap::new())
                .entry(format!("{}-{}", lemma, synset.part_of_speech.str()))
                .or_insert_with(|| Vec::new())
                .push(s2.clone());
            synsets.push(s2);
        }
    }
    let license = "https://github.com/globalwordnet/english-wordnet/blob/master/LICENSE.md";
    let site_url = "https://en-word.net";
    SynsetsHB {
        synsets,
        entries,
        index: index.to_string(),
        name: name.to_string(),
        license,
        site: site_url
    }
}

pub fn lemma_escape(h : &Helper,
                _ : &Handlebars,
                _ : &Context,
                _ : &mut RenderContext,
                out : &mut dyn Output) -> HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(&param.replace(" ", "_"))?;
    Ok(())
}

pub fn long_pos(h : &Helper,
                _ : &Handlebars,
                _ : &Context,
                _ : &mut RenderContext,
                out : &mut dyn Output) -> HelperResult {
    let param = h.param(0)
        .ok_or_else(|| handlebars::RenderErrorReason::ParamNotFoundForName("pos_long", "No parameter for pos_long".to_string()))
        .and_then(|v| {
            let v2 = v.value().as_str()
                .ok_or_else(|| handlebars::RenderErrorReason::ParamTypeMismatchForName("pos_long", "String".to_string(), "something else".to_string()))?;
            crate::wordnet::PartOfSpeech::from_str(v2)
                .map_err(|e| handlebars::RenderErrorReason::Other(format!("{}", e)))
        })?;
    out.write(&param.as_long_string())?;
    Ok(())
}


