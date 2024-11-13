<script>
    import axios from 'axios';

    export default {
        name: "synset",
        props: ["synset", "entries", "display", "focus"],
        data() {
            return {
                show_relations: false,
                show_hypernyms: false,
                show_hyponyms: false,
                show_instance_hypernym: false,
                show_instance_hyponym: false,
                show_antonym: false,
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
            load_targets() {
                let targets = [];
                // add this.synset.hypernym to targets
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
                <span v-if='!example.startsWith("\"")'>&ldquo;</span>{{example}}<span v-if='!example.startsWith("\"")'>&rdquo;</span>
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
                <span v-if="synset.hyponym">
                    <div class="relation-title">
                        <a @click="show_hypernyms = !show_hypernyms">Hypernyms ({{ synset.hypernym.length }})</a>
                    </div>
                    <synset 
                      v-if="show_hypernyms"
                      v-for="hypernym in synset.hypernym"
                      :synset="targetsynsets[hypernym]"
                      :display="display"
                      focus=""
                      :entries="targetentries[hypernym]"></synset>
                </span>
                <span v-if="synset.hyponym">
                    <div class="relation-title">
                        <a @click="show_hyponyms = !show_hyponyms">Hyponyms ({{ synset.hyponym.length }})</a>
                    </div>
                    <synset 
                      v-if="show_hyponyms"
                      v-for="hyponym in synset.hyponym"
                      :synset="targetsynsets[hyponym]"
                      :display="display"
                      focus=""
                      :entries="targetentries[hyponym]"></synset>
                </span>
                <span v-if="synset.instance_hypernym">
                    <div class="relation-title">
                        <a @click="show_instance_hypernym = !show_instance_hypernym">Instance Of ({{ synset.instance_hypernym.length }})</a>
                    </div>
                    <synset 
                      v-if="show_instance_hypernym"
                      v-for="instance_hypernym in synset.instance_hypernym"
                      :synset="targetsynsets[instance_hypernym]"
                      :display="display"
                      focus=""
                      :entries="targetentries[instance_hypernym]"></synset>
                </span>
                <span v-if="synset.instance_hyponym">
                    <div class="relation-title">
                        <a @click="show_instance_hyponym = !show_instance_hyponym">Instances ({{ synset.instance_hyponym.length }})</a>
                    </div>
                    <synset 
                      v-if="show_instance_hyponym"
                      v-for="instance_hyponym in synset.instance_hyponym"
                      :synset="targetsynsets[instance_hyponym]"
                      :display="display"
                      focus=""
                      :entries="targetentries[instance_hyponym]"></synset>
                </span>
                <span v-if="synset.antonym">
                    <div class="relation-title">
                        <a @click="show_antonym = !show_antonym">Antonyms ({{ synset.antonym.length }})</a>
                    </div>
                    <span v-for="antonym in synset.antonym">
                        {{ antonym.source_lemma }} &rarr; {{ antonym.target_lemma }}:
                        <synset 
                          v-if="show_antonym"
                          :synset="targetsynsets[antonym.target_synset]"
                          :display="display"
                          focus=""
                          :entries="targetentries[antonym.target_synset]"></synset>
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
