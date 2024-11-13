relations = [
        ("hypernym", "Hypernyms", False),
        ("hyponym", "Hyponyms", False),
        ("instance_hypernym", "Instance of", False),
        ("instance_hyponym", "Instances", False),
        ("antonym", "Antonyms", True),
        ("also", "See Also", False),
        ("attribute", "Attributes", False),
        ("causes", "Causes", False),
        ("is_caused_by", "Is Caused By", False),
        ("domain_region", "Domains (Region)", False),
        ("has_domain_region", "Has Domain (Region)", False),
        ("domain_topic", "Domains (Topic)", False),
        ("has_domain_topic", "Has Domain (Topic)", False),
        ("exemplifies", "Exemplifies", False),
        ("is_exemplified_by", "Is Exemplified By", False),
        ("examplifies_sense", "Exemplifies", True),
        ("is_exemplified_by_sense", "Is Exemplified By", True),
        ("entails", "Entails", False),
        ("is_entailed_by", "Is Entailed By", False),
        ("mero_member", "Members", False),
        ("holo_member", "Has Member", False),
        ("mero_part", "Parts", False),
        ("holo_part", "Has Part", False),
        ("mero_substance", "Made Up Of", False),
        ("holo_substance", "Makes Up", False),
        ("similar", "Similar", False),
        ("participle", "Participles", True),
        ("is_participle_of", "Is Participle Of", True),
        ("pertainym", "Pertains To", True),
        ("is_pertainym_of", "Is Pertained To By", True),
        ("derivation", "Derivations", True),
        ("agent", "Agents", True),
        ("is_agent_of", "Is Agent Of", True),
        ("material", "Materials", True),
        ("is_material_of", "Is Material Of", True),
        ("event", "Events", True),
        ("is_event_of", "Is Event Of", True),
        ("instrument", "Instruments", True),
        ("is_instrument_of", "Is Instrument Of", True),
        ("location", "Locations", True),
        ("is_location_of", "Is Location Of", True),
        ("by_means_of", "By Means Of", True),
        ("is_by_means_of", "Is By Means Of", True),
        ("undergoer", "Undergoers", True),
        ("is_undergoer_of", "Is Undergoer Of", True),
        ("property", "Properties", True),
        ("is_property_of", "Is Property Of", True),
        ("result", "Results", True),
        ("is_result_of", "Is Result Of", True),
        ("state", "States", True),  
        ("is_state_of", "Is State Of", True),
        ("uses", "Uses", True),
        ("is_used_by", "Is Used By", True),
        ("destination", "Destinations", True),
        ("is_destination_of", "Is Destination Of", True),
        ("body_part", "Body Parts", True),
        ("is_body_part_of", "Is Body Part Of", True),
        ("vehicle", "Vehicles", True),
        ("is_vehicle_of", "Is Vehicle Of", True),
]

with open("vue/src/components/synset.vue", "w") as f:
    f.write("""<script>
    import axios from 'axios';

    export default {
        name: "synset",
        props: ["synset", "entries", "display", "focus"],
        data() {
            return {
                show_relations: false,
""")
    for rel, _, _ in relations:
        f.write(f"                show_{rel}: false,\n")

    f.write("""                targetsynsets: {},
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
            load_targets() {
                let targets = [];
""")
    for rel, _, sense in relations:
        if sense:
            f.write(f"""                for(const relation of this.synset.{rel} || []) {{
                    targets.push(relation.target_synset);
                }}
""")

        else:
            f.write(f"""                for(const relation of this.synset.{rel} || []) {{
                    targets.push(relation);
                }}
""")
    f.write("""                axios.all(targets.map(relation => {
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
            <span class="pos">({{synset.partOfSpeech}})</span>
            <span class="lemma" v-for="(member, index) in synset.members">
                <a target="_self" v-bind:href="'/lemma/' + member" :class="{ underline: member === focus }">{{ member }}</a>
                <span v-if="entryNo(member) > 0"><sup>{{ entryNo(member) }}</sup></span>
                <span v-if="!display.sensekeys && !display.pronunciation && index != synset.members.length - 1">, </span>
                <!--<span v-if="display.pronunciation && sense.pronunciations.length > 0" class="pronunciation">
                    (Pronunciation:
                    <span v-for="pron in sense.pronunciations">
                        <span v-if="pron.variety" class="pronunciation_variety">({{pron.variety}})</span>
                        {{pron.value}}{{$last ? '' : ', '}}</span>)
                </span><span v-if="!display.sensekeys && display.pronunciation">{{$last ? '' : ','}}</span><span v-bind="sense.sense_key" class="sense_key" v-if="display.sensekeys"></span><span v-if="display.sensekeys">{{$last ? '' : ','}}</span>-->
            </span>
            <!--<span v-for="rel in synset.relations">
                <span v-if="'rel_type'=='domain_topic'">
                    ((<i><a v-bind:href="'/id/' + rel.target" target="_blank" v-for="r in targetsynsets"><span v-if="id == rel.target">{{r.lemmas[0].lemma}}</span></a></i>))
                </span>
            </span>-->
            <span class="definition">{{ synset.definition[0] }}</span>
            <span v-for="example in synset.example" class="example"> 
                <span v-if='!example.startsWith("\\"")'>&ldquo;</span>{{example}}<span v-if='!example.startsWith("\\"")'>&rdquo;</span>
            </span>
            <div v-if="display.topics" class="topic">
                <b>Topic: </b> {{ synset.lexname }}
            </div>
            <!--<div ng-if="display.subcats" class="subcats">
                <b>Subcategorization Frames:</b>
                <!--<ul ng-repeat="sense in $ctrl.synset.lemmas" ng-if="$ctrl.hasSubcats()">
                    <li class="subcat" ng-repeat="subcat in sense.subcats">{{$ctrl.replaceSubcat(subcat, sense.lemma)}}</span>
                </ul>--><!--
            </div>-->
            <div v-show="show_relations" class="relations">
""")
    for rel, title, sense in relations:
        f.write(f"""                <span v-if="synset.{rel}">
                    <div class="relation-title">
                        <a @click="show_{rel} = !show_{rel}">{title} ({{{{ synset.{rel}.length }}}})</a>
                    </div>
                    <span v-for="{rel} in synset.{rel}" v-if="show_{rel}">
""")
        if sense:
            f.write(f"""                        {{{{ {rel}.source_lemma }}}} &rarr; {{{{ {rel}.target_lemma }}}}:
                        <synset 
                          :synset="targetsynsets[{rel}.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[{rel}.target_synset]"></synset>
                    </span>
                </span>
""")
        else:
            f.write(f"""                        <synset 
                          :synset="targetsynsets[{rel}]"
                          :display="display"
                          focus=""
                          :entries="targetentries[{rel}]"></synset>
                    </span>
                </span>
""")
    f.write("""
            </div>
            <div class="more">
                <a 
                   v-show="!show_relations"
                   @click="load_targets()">MORE &#x25B6;</a>
            </div>

        </div>
    </div>
</template>""")
 

 


