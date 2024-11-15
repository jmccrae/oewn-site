<script>
    import axios from 'axios';
    import subcat from './subcat.vue';

    export default {
        name: "synset",
        props: ["synset", "entries", "display", "focus", "target_labels"],
        data() {
            return {
                show_relations: false,
                show_hypernym: false,
                show_hyponym: false,
                show_instance_hypernym: false,
                show_instance_hyponym: false,
                show_antonym: false,
                show_also: false,
                show_attribute: false,
                show_causes: false,
                show_is_caused_by: false,
                show_domain_region: false,
                show_has_domain_region: false,
                show_domain_topic: false,
                show_has_domain_topic: false,
                show_exemplifies: false,
                show_is_exemplified_by: false,
                show_examplifies_sense: false,
                show_is_exemplified_by_sense: false,
                show_entails: false,
                show_is_entailed_by: false,
                show_mero_member: false,
                show_holo_member: false,
                show_mero_part: false,
                show_holo_part: false,
                show_mero_substance: false,
                show_holo_substance: false,
                show_similar: false,
                show_participle: false,
                show_is_participle_of: false,
                show_pertainym: false,
                show_is_pertainym_of: false,
                show_derivation: false,
                show_agent: false,
                show_is_agent_of: false,
                show_material: false,
                show_is_material_of: false,
                show_event: false,
                show_is_event_of: false,
                show_instrument: false,
                show_is_instrument_of: false,
                show_location: false,
                show_is_location_of: false,
                show_by_means_of: false,
                show_is_by_means_of: false,
                show_undergoer: false,
                show_is_undergoer_of: false,
                show_property: false,
                show_is_property_of: false,
                show_result: false,
                show_is_result_of: false,
                show_state: false,
                show_is_state_of: false,
                show_uses: false,
                show_is_used_by: false,
                show_destination: false,
                show_is_destination_of: false,
                show_body_part: false,
                show_is_body_part_of: false,
                show_vehicle: false,
                show_is_vehicle_of: false,
                targetsynsets: {},
                targetentries: {}
            }
        },
        methods: {
            entryNo(member) {
                for(const entry of this.entries[member]) {
                    // Any sense of this synset == this.synset.id
                    for(const sense of entry.sense) {
                        if(sense.synset == this.synset.id) {
                            // Return 0 if entry.poskey is a single letter else return the 3rd character
                            return entry.poskey.length == 1 ? 0 : Number(entry.poskey[2]);
                        }
                    }
                }
                return 0;
            },
            pronunciation(member) {
                for (const entry of this.entries[member]) {
                    for (const sense of entry.sense) {
                        if (sense.synset == this.synset.id) {
                            if(entry.pronunciation) {
                                return entry.pronunciation;
                            } else {
                                return [];
                            }
                        }
                    }
                }
                return [];
            },
            sensekey(member) {
                for (const entry of this.entries[member]) {
                    for (const sense of entry.sense) {
                        if (sense.synset == this.synset.id) {
                            return sense.id;
                        }
                    }
                }
                return "";
            },
            subcats() {
                let subcats = {};
                for (const [member, entries] of Object.entries(this.entries)) {
                    for (const entry of entries) {
                        for (const sense of entry.sense) {
                            if (sense.synset == this.synset.id) {
                                if (sense.subcat) {
                                    for (const subcat of sense.subcat) {
                                        if (!subcats[subcat]) {
                                            subcats[subcat] = [];
                                        }
                                        subcats[subcat].push(member);
                                    }
                                }
                            }
                        }
                    }
                }
                return subcats;
            },
            load_targets() {
                let targets = [];
                for(const relation of this.synset.hypernym || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.hyponym || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.instance_hypernym || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.instance_hyponym || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.antonym || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.also || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.attribute || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.causes || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.is_caused_by || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.domain_region || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.has_domain_region || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.domain_topic || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.has_domain_topic || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.exemplifies || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.is_exemplified_by || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.examplifies_sense || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_exemplified_by_sense || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.entails || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.is_entailed_by || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.mero_member || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.holo_member || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.mero_part || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.holo_part || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.mero_substance || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.holo_substance || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.similar || []) {
                    targets.push(relation);
                }
                for(const relation of this.synset.participle || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_participle_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.pertainym || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_pertainym_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.derivation || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.agent || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_agent_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.material || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_material_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.event || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_event_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.instrument || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_instrument_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.location || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_location_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.by_means_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_by_means_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.undergoer || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_undergoer_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.property || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_property_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.result || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_result_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.state || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_state_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.uses || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_used_by || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.destination || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_destination_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.body_part || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_body_part_of || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.vehicle || []) {
                    targets.push(relation.target_synset);
                }
                for(const relation of this.synset.is_vehicle_of || []) {
                    targets.push(relation.target_synset);
                }
                axios.all(targets.map(relation => {
                    return axios.get('/json/id/' + relation);
                }))
                    .then(axios.spread((...responses) => {
                        console.log(JSON.stringify(responses));
                        for(var i = 0; i < responses.length; i++) {
                            let ssid = responses[i].data.synsets[0].id;
                            this.targetsynsets[ssid] = responses[i].data.synsets[0];
                            this.targetentries[ssid] = responses[i].data.entries;
                        }
                        this.show_relations = true;
                    }))
                    .catch(error => {
                        console.log(error);
                    });
            }
        },
        components: {
            subcat
        },
    }
</script>

<template>
    <div class="synset">
        <div class="synset-id" v-show="display.ids">
            <span class="identifier">{{ synset.id }}</span> (<b class="synset-id-title">Interlingual Index:</b> 
            <span class="identifier">{{ synset.ili }}</span>)
            <hr/>
        </div>

        <div class="lemmas">
            <span class="pos">({{synset.partOfSpeech}}) </span>
            <span class="lemma" v-for="(member, index) in synset.members">
                <a target="_self" v-bind:href="'/lemma/' + member" :class="{ underline: member === focus }">{{ member }}</a>
                <span v-if="entryNo(member) > 0"><sup>{{ entryNo(member) }}</sup></span>
                <span v-if="display.sensekeys" class="sense_key"> {{ sensekey(member) }}</span>
                <span v-if="display.pronunciation && pronunciation(member).length > 0" class="pronunciation">
                    (Pronunciation:
                    <span v-for="(pron, index) in pronunciation(member)">
                        <span v-if="pron.variety" class="pronunciation_variety">({{pron.variety}})</span>
                        {{pron.value}}{{index == pronunciation(member).length - 1 ? '' : ', '}}
                    </span>)&nbsp;
                </span>
                <span v-if="index != synset.members.length - 1">, </span>
                <span v-if="index == synset.members.length - 1"> </span>
            </span>
            <span v-for="rel in synset.domain_topic || []">
                ((<i><a v-bind:href="'/id/' + rel" target="_blank">{{ target_labels[rel] }}</a></i>))
            </span>
            <span class="definition">{{ synset.definition[0] }}</span>
            <span v-for="example in synset.example" class="example"> 
                <span v-if='!example.startsWith("\"")'>&ldquo;</span>{{example}}<span v-if='!example.startsWith("\"")'>&rdquo;</span>
            </span>
            <div v-if="display.topics" class="topic">
                <b>Topic: </b> {{ synset.lexname }}
            </div>
            <subcat v-if="display.subcats"
                    :subcats="subcats()"></subcat>
            <div v-show="show_relations" class="relations">
                <span v-if="synset.hypernym">
                    <div class="relation-title">
                        <a @click="show_hypernym = !show_hypernym">Hypernyms ({{ synset.hypernym.length }})</a>
                    </div>
                    <span v-for="hypernym in synset.hypernym" v-if="show_hypernym">
                        <synset 
                          :synset="targetsynsets[hypernym]"
                          :display="display"
                          focus=""
                          :entries="targetentries[hypernym]"></synset>
                    </span>
                </span>
                <span v-if="synset.hyponym">
                    <div class="relation-title">
                        <a @click="show_hyponym = !show_hyponym">Hyponyms ({{ synset.hyponym.length }})</a>
                    </div>
                    <span v-for="hyponym in synset.hyponym" v-if="show_hyponym">
                        <synset 
                          :synset="targetsynsets[hyponym]"
                          :display="display"
                          focus=""
                          :entries="targetentries[hyponym]"></synset>
                    </span>
                </span>
                <span v-if="synset.instance_hypernym">
                    <div class="relation-title">
                        <a @click="show_instance_hypernym = !show_instance_hypernym">Instance of ({{ synset.instance_hypernym.length }})</a>
                    </div>
                    <span v-for="instance_hypernym in synset.instance_hypernym" v-if="show_instance_hypernym">
                        <synset 
                          :synset="targetsynsets[instance_hypernym]"
                          :display="display"
                          focus=""
                          :entries="targetentries[instance_hypernym]"></synset>
                    </span>
                </span>
                <span v-if="synset.instance_hyponym">
                    <div class="relation-title">
                        <a @click="show_instance_hyponym = !show_instance_hyponym">Instances ({{ synset.instance_hyponym.length }})</a>
                    </div>
                    <span v-for="instance_hyponym in synset.instance_hyponym" v-if="show_instance_hyponym">
                        <synset 
                          :synset="targetsynsets[instance_hyponym]"
                          :display="display"
                          focus=""
                          :entries="targetentries[instance_hyponym]"></synset>
                    </span>
                </span>
                <span v-if="synset.antonym">
                    <div class="relation-title">
                        <a @click="show_antonym = !show_antonym">Antonyms ({{ synset.antonym.length }})</a>
                    </div>
                    <span v-for="antonym in synset.antonym" v-if="show_antonym">
                        {{ antonym.source_lemma }} &rarr; {{ antonym.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[antonym.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[antonym.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.also">
                    <div class="relation-title">
                        <a @click="show_also = !show_also">See Also ({{ synset.also.length }})</a>
                    </div>
                    <span v-for="also in synset.also" v-if="show_also">
                        <synset 
                          :synset="targetsynsets[also]"
                          :display="display"
                          focus=""
                          :entries="targetentries[also]"></synset>
                    </span>
                </span>
                <span v-if="synset.attribute">
                    <div class="relation-title">
                        <a @click="show_attribute = !show_attribute">Attributes ({{ synset.attribute.length }})</a>
                    </div>
                    <span v-for="attribute in synset.attribute" v-if="show_attribute">
                        <synset 
                          :synset="targetsynsets[attribute]"
                          :display="display"
                          focus=""
                          :entries="targetentries[attribute]"></synset>
                    </span>
                </span>
                <span v-if="synset.causes">
                    <div class="relation-title">
                        <a @click="show_causes = !show_causes">Causes ({{ synset.causes.length }})</a>
                    </div>
                    <span v-for="causes in synset.causes" v-if="show_causes">
                        <synset 
                          :synset="targetsynsets[causes]"
                          :display="display"
                          focus=""
                          :entries="targetentries[causes]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_caused_by">
                    <div class="relation-title">
                        <a @click="show_is_caused_by = !show_is_caused_by">Is Caused By ({{ synset.is_caused_by.length }})</a>
                    </div>
                    <span v-for="is_caused_by in synset.is_caused_by" v-if="show_is_caused_by">
                        <synset 
                          :synset="targetsynsets[is_caused_by]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_caused_by]"></synset>
                    </span>
                </span>
                <span v-if="synset.domain_region">
                    <div class="relation-title">
                        <a @click="show_domain_region = !show_domain_region">Domains (Region) ({{ synset.domain_region.length }})</a>
                    </div>
                    <span v-for="domain_region in synset.domain_region" v-if="show_domain_region">
                        <synset 
                          :synset="targetsynsets[domain_region]"
                          :display="display"
                          focus=""
                          :entries="targetentries[domain_region]"></synset>
                    </span>
                </span>
                <span v-if="synset.has_domain_region">
                    <div class="relation-title">
                        <a @click="show_has_domain_region = !show_has_domain_region">Has Domain (Region) ({{ synset.has_domain_region.length }})</a>
                    </div>
                    <span v-for="has_domain_region in synset.has_domain_region" v-if="show_has_domain_region">
                        <synset 
                          :synset="targetsynsets[has_domain_region]"
                          :display="display"
                          focus=""
                          :entries="targetentries[has_domain_region]"></synset>
                    </span>
                </span>
                <span v-if="synset.domain_topic">
                    <div class="relation-title">
                        <a @click="show_domain_topic = !show_domain_topic">Domains (Topic) ({{ synset.domain_topic.length }})</a>
                    </div>
                    <span v-for="domain_topic in synset.domain_topic" v-if="show_domain_topic">
                        <synset 
                          :synset="targetsynsets[domain_topic]"
                          :display="display"
                          focus=""
                          :entries="targetentries[domain_topic]"></synset>
                    </span>
                </span>
                <span v-if="synset.has_domain_topic">
                    <div class="relation-title">
                        <a @click="show_has_domain_topic = !show_has_domain_topic">Has Domain (Topic) ({{ synset.has_domain_topic.length }})</a>
                    </div>
                    <span v-for="has_domain_topic in synset.has_domain_topic" v-if="show_has_domain_topic">
                        <synset 
                          :synset="targetsynsets[has_domain_topic]"
                          :display="display"
                          focus=""
                          :entries="targetentries[has_domain_topic]"></synset>
                    </span>
                </span>
                <span v-if="synset.exemplifies">
                    <div class="relation-title">
                        <a @click="show_exemplifies = !show_exemplifies">Exemplifies ({{ synset.exemplifies.length }})</a>
                    </div>
                    <span v-for="exemplifies in synset.exemplifies" v-if="show_exemplifies">
                        <synset 
                          :synset="targetsynsets[exemplifies]"
                          :display="display"
                          focus=""
                          :entries="targetentries[exemplifies]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_exemplified_by">
                    <div class="relation-title">
                        <a @click="show_is_exemplified_by = !show_is_exemplified_by">Is Exemplified By ({{ synset.is_exemplified_by.length }})</a>
                    </div>
                    <span v-for="is_exemplified_by in synset.is_exemplified_by" v-if="show_is_exemplified_by">
                        <synset 
                          :synset="targetsynsets[is_exemplified_by]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_exemplified_by]"></synset>
                    </span>
                </span>
                <span v-if="synset.examplifies_sense">
                    <div class="relation-title">
                        <a @click="show_examplifies_sense = !show_examplifies_sense">Exemplifies ({{ synset.examplifies_sense.length }})</a>
                    </div>
                    <span v-for="examplifies_sense in synset.examplifies_sense" v-if="show_examplifies_sense">
                        {{ examplifies_sense.source_lemma }} &rarr; {{ examplifies_sense.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[examplifies_sense.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[examplifies_sense.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_exemplified_by_sense">
                    <div class="relation-title">
                        <a @click="show_is_exemplified_by_sense = !show_is_exemplified_by_sense">Is Exemplified By ({{ synset.is_exemplified_by_sense.length }})</a>
                    </div>
                    <span v-for="is_exemplified_by_sense in synset.is_exemplified_by_sense" v-if="show_is_exemplified_by_sense">
                        {{ is_exemplified_by_sense.source_lemma }} &rarr; {{ is_exemplified_by_sense.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_exemplified_by_sense.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_exemplified_by_sense.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.entails">
                    <div class="relation-title">
                        <a @click="show_entails = !show_entails">Entails ({{ synset.entails.length }})</a>
                    </div>
                    <span v-for="entails in synset.entails" v-if="show_entails">
                        <synset 
                          :synset="targetsynsets[entails]"
                          :display="display"
                          focus=""
                          :entries="targetentries[entails]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_entailed_by">
                    <div class="relation-title">
                        <a @click="show_is_entailed_by = !show_is_entailed_by">Is Entailed By ({{ synset.is_entailed_by.length }})</a>
                    </div>
                    <span v-for="is_entailed_by in synset.is_entailed_by" v-if="show_is_entailed_by">
                        <synset 
                          :synset="targetsynsets[is_entailed_by]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_entailed_by]"></synset>
                    </span>
                </span>
                <span v-if="synset.mero_member">
                    <div class="relation-title">
                        <a @click="show_mero_member = !show_mero_member">Members ({{ synset.mero_member.length }})</a>
                    </div>
                    <span v-for="mero_member in synset.mero_member" v-if="show_mero_member">
                        <synset 
                          :synset="targetsynsets[mero_member]"
                          :display="display"
                          focus=""
                          :entries="targetentries[mero_member]"></synset>
                    </span>
                </span>
                <span v-if="synset.holo_member">
                    <div class="relation-title">
                        <a @click="show_holo_member = !show_holo_member">Has Member ({{ synset.holo_member.length }})</a>
                    </div>
                    <span v-for="holo_member in synset.holo_member" v-if="show_holo_member">
                        <synset 
                          :synset="targetsynsets[holo_member]"
                          :display="display"
                          focus=""
                          :entries="targetentries[holo_member]"></synset>
                    </span>
                </span>
                <span v-if="synset.mero_part">
                    <div class="relation-title">
                        <a @click="show_mero_part = !show_mero_part">Parts ({{ synset.mero_part.length }})</a>
                    </div>
                    <span v-for="mero_part in synset.mero_part" v-if="show_mero_part">
                        <synset 
                          :synset="targetsynsets[mero_part]"
                          :display="display"
                          focus=""
                          :entries="targetentries[mero_part]"></synset>
                    </span>
                </span>
                <span v-if="synset.holo_part">
                    <div class="relation-title">
                        <a @click="show_holo_part = !show_holo_part">Has Part ({{ synset.holo_part.length }})</a>
                    </div>
                    <span v-for="holo_part in synset.holo_part" v-if="show_holo_part">
                        <synset 
                          :synset="targetsynsets[holo_part]"
                          :display="display"
                          focus=""
                          :entries="targetentries[holo_part]"></synset>
                    </span>
                </span>
                <span v-if="synset.mero_substance">
                    <div class="relation-title">
                        <a @click="show_mero_substance = !show_mero_substance">Made Up Of ({{ synset.mero_substance.length }})</a>
                    </div>
                    <span v-for="mero_substance in synset.mero_substance" v-if="show_mero_substance">
                        <synset 
                          :synset="targetsynsets[mero_substance]"
                          :display="display"
                          focus=""
                          :entries="targetentries[mero_substance]"></synset>
                    </span>
                </span>
                <span v-if="synset.holo_substance">
                    <div class="relation-title">
                        <a @click="show_holo_substance = !show_holo_substance">Makes Up ({{ synset.holo_substance.length }})</a>
                    </div>
                    <span v-for="holo_substance in synset.holo_substance" v-if="show_holo_substance">
                        <synset 
                          :synset="targetsynsets[holo_substance]"
                          :display="display"
                          focus=""
                          :entries="targetentries[holo_substance]"></synset>
                    </span>
                </span>
                <span v-if="synset.similar">
                    <div class="relation-title">
                        <a @click="show_similar = !show_similar">Similar ({{ synset.similar.length }})</a>
                    </div>
                    <span v-for="similar in synset.similar" v-if="show_similar">
                        <synset 
                          :synset="targetsynsets[similar]"
                          :display="display"
                          focus=""
                          :entries="targetentries[similar]"></synset>
                    </span>
                </span>
                <span v-if="synset.participle">
                    <div class="relation-title">
                        <a @click="show_participle = !show_participle">Participles ({{ synset.participle.length }})</a>
                    </div>
                    <span v-for="participle in synset.participle" v-if="show_participle">
                        {{ participle.source_lemma }} &rarr; {{ participle.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[participle.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[participle.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_participle_of">
                    <div class="relation-title">
                        <a @click="show_is_participle_of = !show_is_participle_of">Is Participle Of ({{ synset.is_participle_of.length }})</a>
                    </div>
                    <span v-for="is_participle_of in synset.is_participle_of" v-if="show_is_participle_of">
                        {{ is_participle_of.source_lemma }} &rarr; {{ is_participle_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_participle_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_participle_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.pertainym">
                    <div class="relation-title">
                        <a @click="show_pertainym = !show_pertainym">Pertains To ({{ synset.pertainym.length }})</a>
                    </div>
                    <span v-for="pertainym in synset.pertainym" v-if="show_pertainym">
                        {{ pertainym.source_lemma }} &rarr; {{ pertainym.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[pertainym.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[pertainym.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_pertainym_of">
                    <div class="relation-title">
                        <a @click="show_is_pertainym_of = !show_is_pertainym_of">Is Pertained To By ({{ synset.is_pertainym_of.length }})</a>
                    </div>
                    <span v-for="is_pertainym_of in synset.is_pertainym_of" v-if="show_is_pertainym_of">
                        {{ is_pertainym_of.source_lemma }} &rarr; {{ is_pertainym_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_pertainym_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_pertainym_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.derivation">
                    <div class="relation-title">
                        <a @click="show_derivation = !show_derivation">Derivations ({{ synset.derivation.length }})</a>
                    </div>
                    <span v-for="derivation in synset.derivation" v-if="show_derivation">
                        {{ derivation.source_lemma }} &rarr; {{ derivation.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[derivation.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[derivation.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.agent">
                    <div class="relation-title">
                        <a @click="show_agent = !show_agent">Agents ({{ synset.agent.length }})</a>
                    </div>
                    <span v-for="agent in synset.agent" v-if="show_agent">
                        {{ agent.source_lemma }} &rarr; {{ agent.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[agent.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[agent.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_agent_of">
                    <div class="relation-title">
                        <a @click="show_is_agent_of = !show_is_agent_of">Is Agent Of ({{ synset.is_agent_of.length }})</a>
                    </div>
                    <span v-for="is_agent_of in synset.is_agent_of" v-if="show_is_agent_of">
                        {{ is_agent_of.source_lemma }} &rarr; {{ is_agent_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_agent_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_agent_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.material">
                    <div class="relation-title">
                        <a @click="show_material = !show_material">Materials ({{ synset.material.length }})</a>
                    </div>
                    <span v-for="material in synset.material" v-if="show_material">
                        {{ material.source_lemma }} &rarr; {{ material.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[material.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[material.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_material_of">
                    <div class="relation-title">
                        <a @click="show_is_material_of = !show_is_material_of">Is Material Of ({{ synset.is_material_of.length }})</a>
                    </div>
                    <span v-for="is_material_of in synset.is_material_of" v-if="show_is_material_of">
                        {{ is_material_of.source_lemma }} &rarr; {{ is_material_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_material_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_material_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.event">
                    <div class="relation-title">
                        <a @click="show_event = !show_event">Events ({{ synset.event.length }})</a>
                    </div>
                    <span v-for="event in synset.event" v-if="show_event">
                        {{ event.source_lemma }} &rarr; {{ event.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[event.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[event.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_event_of">
                    <div class="relation-title">
                        <a @click="show_is_event_of = !show_is_event_of">Is Event Of ({{ synset.is_event_of.length }})</a>
                    </div>
                    <span v-for="is_event_of in synset.is_event_of" v-if="show_is_event_of">
                        {{ is_event_of.source_lemma }} &rarr; {{ is_event_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_event_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_event_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.instrument">
                    <div class="relation-title">
                        <a @click="show_instrument = !show_instrument">Instruments ({{ synset.instrument.length }})</a>
                    </div>
                    <span v-for="instrument in synset.instrument" v-if="show_instrument">
                        {{ instrument.source_lemma }} &rarr; {{ instrument.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[instrument.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[instrument.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_instrument_of">
                    <div class="relation-title">
                        <a @click="show_is_instrument_of = !show_is_instrument_of">Is Instrument Of ({{ synset.is_instrument_of.length }})</a>
                    </div>
                    <span v-for="is_instrument_of in synset.is_instrument_of" v-if="show_is_instrument_of">
                        {{ is_instrument_of.source_lemma }} &rarr; {{ is_instrument_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_instrument_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_instrument_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.location">
                    <div class="relation-title">
                        <a @click="show_location = !show_location">Locations ({{ synset.location.length }})</a>
                    </div>
                    <span v-for="location in synset.location" v-if="show_location">
                        {{ location.source_lemma }} &rarr; {{ location.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[location.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[location.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_location_of">
                    <div class="relation-title">
                        <a @click="show_is_location_of = !show_is_location_of">Is Location Of ({{ synset.is_location_of.length }})</a>
                    </div>
                    <span v-for="is_location_of in synset.is_location_of" v-if="show_is_location_of">
                        {{ is_location_of.source_lemma }} &rarr; {{ is_location_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_location_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_location_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.by_means_of">
                    <div class="relation-title">
                        <a @click="show_by_means_of = !show_by_means_of">By Means Of ({{ synset.by_means_of.length }})</a>
                    </div>
                    <span v-for="by_means_of in synset.by_means_of" v-if="show_by_means_of">
                        {{ by_means_of.source_lemma }} &rarr; {{ by_means_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[by_means_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[by_means_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_by_means_of">
                    <div class="relation-title">
                        <a @click="show_is_by_means_of = !show_is_by_means_of">Is By Means Of ({{ synset.is_by_means_of.length }})</a>
                    </div>
                    <span v-for="is_by_means_of in synset.is_by_means_of" v-if="show_is_by_means_of">
                        {{ is_by_means_of.source_lemma }} &rarr; {{ is_by_means_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_by_means_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_by_means_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.undergoer">
                    <div class="relation-title">
                        <a @click="show_undergoer = !show_undergoer">Undergoers ({{ synset.undergoer.length }})</a>
                    </div>
                    <span v-for="undergoer in synset.undergoer" v-if="show_undergoer">
                        {{ undergoer.source_lemma }} &rarr; {{ undergoer.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[undergoer.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[undergoer.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_undergoer_of">
                    <div class="relation-title">
                        <a @click="show_is_undergoer_of = !show_is_undergoer_of">Is Undergoer Of ({{ synset.is_undergoer_of.length }})</a>
                    </div>
                    <span v-for="is_undergoer_of in synset.is_undergoer_of" v-if="show_is_undergoer_of">
                        {{ is_undergoer_of.source_lemma }} &rarr; {{ is_undergoer_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_undergoer_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_undergoer_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.property">
                    <div class="relation-title">
                        <a @click="show_property = !show_property">Properties ({{ synset.property.length }})</a>
                    </div>
                    <span v-for="property in synset.property" v-if="show_property">
                        {{ property.source_lemma }} &rarr; {{ property.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[property.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[property.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_property_of">
                    <div class="relation-title">
                        <a @click="show_is_property_of = !show_is_property_of">Is Property Of ({{ synset.is_property_of.length }})</a>
                    </div>
                    <span v-for="is_property_of in synset.is_property_of" v-if="show_is_property_of">
                        {{ is_property_of.source_lemma }} &rarr; {{ is_property_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_property_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_property_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.result">
                    <div class="relation-title">
                        <a @click="show_result = !show_result">Results ({{ synset.result.length }})</a>
                    </div>
                    <span v-for="result in synset.result" v-if="show_result">
                        {{ result.source_lemma }} &rarr; {{ result.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[result.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[result.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_result_of">
                    <div class="relation-title">
                        <a @click="show_is_result_of = !show_is_result_of">Is Result Of ({{ synset.is_result_of.length }})</a>
                    </div>
                    <span v-for="is_result_of in synset.is_result_of" v-if="show_is_result_of">
                        {{ is_result_of.source_lemma }} &rarr; {{ is_result_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_result_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_result_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.state">
                    <div class="relation-title">
                        <a @click="show_state = !show_state">States ({{ synset.state.length }})</a>
                    </div>
                    <span v-for="state in synset.state" v-if="show_state">
                        {{ state.source_lemma }} &rarr; {{ state.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[state.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[state.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_state_of">
                    <div class="relation-title">
                        <a @click="show_is_state_of = !show_is_state_of">Is State Of ({{ synset.is_state_of.length }})</a>
                    </div>
                    <span v-for="is_state_of in synset.is_state_of" v-if="show_is_state_of">
                        {{ is_state_of.source_lemma }} &rarr; {{ is_state_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_state_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_state_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.uses">
                    <div class="relation-title">
                        <a @click="show_uses = !show_uses">Uses ({{ synset.uses.length }})</a>
                    </div>
                    <span v-for="uses in synset.uses" v-if="show_uses">
                        {{ uses.source_lemma }} &rarr; {{ uses.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[uses.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[uses.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_used_by">
                    <div class="relation-title">
                        <a @click="show_is_used_by = !show_is_used_by">Is Used By ({{ synset.is_used_by.length }})</a>
                    </div>
                    <span v-for="is_used_by in synset.is_used_by" v-if="show_is_used_by">
                        {{ is_used_by.source_lemma }} &rarr; {{ is_used_by.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_used_by.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_used_by.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.destination">
                    <div class="relation-title">
                        <a @click="show_destination = !show_destination">Destinations ({{ synset.destination.length }})</a>
                    </div>
                    <span v-for="destination in synset.destination" v-if="show_destination">
                        {{ destination.source_lemma }} &rarr; {{ destination.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[destination.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[destination.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_destination_of">
                    <div class="relation-title">
                        <a @click="show_is_destination_of = !show_is_destination_of">Is Destination Of ({{ synset.is_destination_of.length }})</a>
                    </div>
                    <span v-for="is_destination_of in synset.is_destination_of" v-if="show_is_destination_of">
                        {{ is_destination_of.source_lemma }} &rarr; {{ is_destination_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_destination_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_destination_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.body_part">
                    <div class="relation-title">
                        <a @click="show_body_part = !show_body_part">Body Parts ({{ synset.body_part.length }})</a>
                    </div>
                    <span v-for="body_part in synset.body_part" v-if="show_body_part">
                        {{ body_part.source_lemma }} &rarr; {{ body_part.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[body_part.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[body_part.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_body_part_of">
                    <div class="relation-title">
                        <a @click="show_is_body_part_of = !show_is_body_part_of">Is Body Part Of ({{ synset.is_body_part_of.length }})</a>
                    </div>
                    <span v-for="is_body_part_of in synset.is_body_part_of" v-if="show_is_body_part_of">
                        {{ is_body_part_of.source_lemma }} &rarr; {{ is_body_part_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_body_part_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_body_part_of.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.vehicle">
                    <div class="relation-title">
                        <a @click="show_vehicle = !show_vehicle">Vehicles ({{ synset.vehicle.length }})</a>
                    </div>
                    <span v-for="vehicle in synset.vehicle" v-if="show_vehicle">
                        {{ vehicle.source_lemma }} &rarr; {{ vehicle.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[vehicle.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[vehicle.target_synset]"></synset>
                    </span>
                </span>
                <span v-if="synset.is_vehicle_of">
                    <div class="relation-title">
                        <a @click="show_is_vehicle_of = !show_is_vehicle_of">Is Vehicle Of ({{ synset.is_vehicle_of.length }})</a>
                    </div>
                    <span v-for="is_vehicle_of in synset.is_vehicle_of" v-if="show_is_vehicle_of">
                        {{ is_vehicle_of.source_lemma }} &rarr; {{ is_vehicle_of.target_lemma }}:
                        <synset 
                          :synset="targetsynsets[is_vehicle_of.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[is_vehicle_of.target_synset]"></synset>
                    </span>
                </span>

            </div>
            <div class="more">
                <a 
                   v-show="!show_relations"
                   @click="load_targets()">MORE &#x25B6;</a>
            </div>

        </div>
    </div>
</template>
