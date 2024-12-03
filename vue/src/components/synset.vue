<script>
    import axios from 'axios';
    import subcat from './subcat.vue';

    export default {
        name: "synset",
        props: ["synset", "display", "focus", "target_labels"],
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
                show_exemplifies_sense: false,
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
                targetentries: {},
                targetlabels: {},
                relation_list: [
                    {"name":"hypernym", "title":"Hypernyms", "sense":false},
                    {"name":"hyponym", "title":"Hyponyms", "sense":false},
                    {"name":"instance_hypernym", "title":"Instance of", "sense":false},
                    {"name":"instance_hyponym", "title":"Instances", "sense":false},
                    {"name":"antonym", "title":"Antonyms", "sense":true},
                    {"name":"also", "title":"See Also", "sense":false},
                    {"name":"attribute", "title":"Attributes", "sense":false},
                    {"name":"causes", "title":"Causes", "sense":false},
                    {"name":"is_caused_by", "title":"Is Caused By", "sense":false},
                    {"name":"domain_region", "title":"Domains (Region)", "sense":false},
                    {"name":"has_domain_region", "title":"Has Domain (Region)", "sense":false},
                    {"name":"domain_topic", "title":"Domains (Topic)", "sense":false},
                    {"name":"has_domain_topic", "title":"Has Domain (Topic)", "sense":false},
                    {"name":"exemplifies", "title":"Exemplifies", "sense":false},
                    {"name":"is_exemplified_by", "title":"Is Exemplified By", "sense":false},
                    {"name":"exemplifies_sense", "title":"Exemplifies", "sense":true},
                    {"name":"is_exemplified_by_sense", "title":"Is Exemplified By", "sense":true},
                    {"name":"entails", "title":"Entails", "sense":false},
                    {"name":"is_entailed_by", "title":"Is Entailed By", "sense":false},
                    {"name":"mero_member", "title":"Members", "sense":false},
                    {"name":"holo_member", "title":"Has Member", "sense":false},
                    {"name":"mero_part", "title":"Parts", "sense":false},
                    {"name":"holo_part", "title":"Has Part", "sense":false},
                    {"name":"mero_substance", "title":"Made Up Of", "sense":false},
                    {"name":"holo_substance", "title":"Makes Up", "sense":false},
                    {"name":"similar", "title":"Similar", "sense":false},
                    {"name":"participle", "title":"Participles", "sense":true},
                    {"name":"is_participle_of", "title":"Is Participle Of", "sense":true},
                    {"name":"pertainym", "title":"Pertains To", "sense":true},
                    {"name":"is_pertainym_of", "title":"Is Pertained To By", "sense":true},
                    {"name":"derivation", "title":"Derivations", "sense":true},
                    {"name":"agent", "title":"Agents", "sense":true},
                    {"name":"is_agent_of", "title":"Is Agent Of", "sense":true},
                    {"name":"material", "title":"Materials", "sense":true},
                    {"name":"is_material_of", "title":"Is Material Of", "sense":true},
                    {"name":"event", "title":"Events", "sense":true},
                    {"name":"is_event_of", "title":"Is Event Of", "sense":true},
                    {"name":"instrument", "title":"Instruments", "sense":true},
                    {"name":"is_instrument_of", "title":"Is Instrument Of", "sense":true},
                    {"name":"location", "title":"Locations", "sense":true},
                    {"name":"is_location_of", "title":"Is Location Of", "sense":true},
                    {"name":"by_means_of", "title":"By Means Of", "sense":true},
                    {"name":"is_by_means_of", "title":"Is By Means Of", "sense":true},
                    {"name":"undergoer", "title":"Undergoers", "sense":true},
                    {"name":"is_undergoer_of", "title":"Is Undergoer Of", "sense":true},
                    {"name":"property", "title":"Properties", "sense":true},
                    {"name":"is_property_of", "title":"Is Property Of", "sense":true},
                    {"name":"result", "title":"Results", "sense":true},
                    {"name":"is_result_of", "title":"Is Result Of", "sense":true},
                    {"name":"state", "title":"States", "sense":true},  
                    {"name":"is_state_of", "title":"Is State Of", "sense":true},
                    {"name":"uses", "title":"Uses", "sense":true},
                    {"name":"is_used_by", "title":"Is Used By", "sense":true},
                    {"name":"destination", "title":"Destinations", "sense":true},
                    {"name":"is_destination_of", "title":"Is Destination Of", "sense":true},
                    {"name":"body_part", "title":"Body Parts", "sense":true},
                    {"name":"is_body_part_of", "title":"Is Body Part Of", "sense":true},
                    {"name":"vehicle", "title":"Vehicles", "sense":true},
                    {"name":"is_vehicle_of", "title":"Is Vehicle Of", "sense":true}
                ]

            }
        },
        methods: {
            subcats() {
                let subcats = {};
                for (const member of this.synset.members) {
                    for (const subcat of member.sense.subcat) {
                        if (!subcats[subcat]) {
                            subcats[subcat] = [];
                        }
                        subcats[subcat].push(member.lemma);
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
                for(const relation of this.synset.exemplifies_sense || []) {
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
                let query = '/json/ids?';
                for(const rel of targets) {
                    query += 'id=' + rel + '&';
                }
                query = query.slice(0, -1);
                axios.get(query)
                    .then(response => {
                        this.targetsynsets = {};
                        for(const synset of response.data.synsets) {
                            this.targetsynsets[synset.id] = synset;
                        }
                        this.targetlabels = response.data.target_labels;
                        this.show_relations = true;
                    })
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
            <span class="identifier">{{ synset.id }}</span> 
            <span v-if="synset.ili || synset.wikidata">(<b class="synset-id-title" v-if="synset.ili">Interlingual Index:</b> 
            <span class="identifier">{{ synset.ili }}</span><span v-if="synset.ili && synset.wikidata">, </span>
            <span v-if="synset.wikidata"><b>Wikidata:</b> <a v-bind:href="'https://www.wikidata.org/entity/' + synset.wikidata" target="_blank">{{ synset.wikidata }}</a></span>)</span>
            <hr/>
        </div>

        <div class="lemmas">
            <span class="pos">({{synset.partOfSpeech}}) </span>
            <span class="lemma" v-for="(member, index) in synset.members">
                <a target="_self" v-bind:href="'/lemma/' + member.lemma" :class="{ underline: member.lemma === focus }">{{ member.lemma }}</a>
                <span v-if="'entry_no' in member"><sup>{{ member.entry_no }}</sup></span>
                <span v-if="display.sensekeys" class="sense_key"> {{ member.sense.id }}</span>
                <span v-if="display.pronunciation && 'pronunciation' in member" class="pronunciation">
                    (Pronunciation:
                    <span v-for="(pron, index) in member.pronunciation">
                        <span v-if="pron.variety" class="pronunciation_variety">({{pron.variety}})</span>
                        {{pron.value}}{{index == member.pronunciation.length - 1 ? '' : ', '}}
                    </span>)&nbsp;
                </span>
                <span v-if="index != synset.members.length - 1">, </span>
                <span v-if="index == synset.members.length - 1"> </span>
            </span>&nbsp;
            <span v-for="rel in synset.domain_topic || []">
                ((<i><a v-bind:href="'/id/' + rel" target="_blank">{{ target_labels[rel] }}</a></i>))
            </span>
            <span class="definition">{{ synset.definition[0] }}</span>&nbsp;
            <span v-for="(example, index) in synset.example" class="example">
                <span v-if='typeof example === "string"'>&ldquo;{{example}}&rdquo;</span>
                <span v-if='typeof example === "object" && example.source.startsWith("http")'>&ldquo;<a v-bind:href="example.source" target="_blank">{{example.text}}</a>&rdquo;</span>
                <span v-if='typeof example === "object" && !example.source.startsWith("http")'>&ldquo;{{example.text}}&rdquo; ({{example.source}})</span>
                <span v-if="index != synset.example.length - 1">, </span>
            </span>
            <div v-if="display.topics" class="topic">
                <b>Topic: </b> {{ synset.lexname }}
            </div>
            <subcat v-if="display.subcats"
                    :subcats="subcats()"></subcat>
            <div v-show="show_relations" class="relations">
                <span v-for="relation_info in relation_list">
                    <span v-if="relation_info.name in synset">
                        <div class="relation-title">
                            <a @click="this['show_' + relation_info.name] = !this['show_' + relation_info.name]">{{ relation_info.title }} ({{ synset[relation_info.name].length }})</a>
                        </div>
                        <span v-for="relation in synset[relation_info.name]" v-if="this['show_' + relation_info.name]">
                            <span v-if="relation_info.sense">
                                {{ relation.source_lemma }} &rarr; {{ relation.target_lemma }}:
                                <synset 
                                  :synset="targetsynsets[relation.target_synset]"
                                  :display="display"
                                  focus=""
                                  :target_labels="targetlabels"></synset>
                            </span>
                            <span v-if="!relation_info.sense">
                                <synset 
                                  :synset="targetsynsets[relation]"
                                  :display="display"
                                  focus=""
                                  :target_labels="targetlabels"></synset>
                            </span>
                        </span>
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
