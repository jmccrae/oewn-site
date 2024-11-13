<script>
    import axios from 'axios';

    export default {
        name: "synset",
        props: ["synset", "entries", "display", "focus"],
        data() {
            return {
                show_relations: false,
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
                axios.all(this.synset.hypernym.map(relation => {
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
            <div v-if="show_relations" class="relations">
                <div class="relation-title">
                    <a href="#" @click="show_hypernyms = !show_hypernyms">Hypernyms ({{ synset.hypernym.length }})</a>
                </div>
                <span v-for="hypernym in synset.hypernym">
                    <synset :synset="targetsynsets[hypernym]"
                      :display="display"
                      focus=""
                      :entries="targetentries[hypernym]"></synset>
                </span>
                <!--<relation relation="hyponym" display="$ctrl.display"
                                             targetsynsets="$ctrl.targetsynsets"
                                             relations="$ctrl.synset.relations"
                                             fullname="Hyponyms"></relation>
                <relation relation="antonym" display="$ctrl.display"
                                             targetsynsets="$ctrl.targetsynsets"
                                             relations="$ctrl.synset.relations"
                                             fullname="Antonyms"></relation>
                <relation relation="attribute" display="$ctrl.display"
                                               targetsynsets="$ctrl.targetsynsets"
                                               relations="$ctrl.synset.relations"
                                               fullname="Attributes"></relation>
                <relation relation="causes" display="$ctrl.display"
                                            targetsynsets="$ctrl.targetsynsets"
                                            relations="$ctrl.synset.relations"
                                            fullname="Causes"></relation>
                <relation relation="derivation" display="$ctrl.display"
                                                targetsynsets="$ctrl.targetsynsets"
                                                relations="$ctrl.synset.relations"
                                                fullname="Derived Forms"></relation>
                <relation relation="domain_category" display="$ctrl.display"
                                                     targetsynsets="$ctrl.targetsynsets"
                                                     relations="$ctrl.synset.relations"
                                                     fullname="Domains (category)"></relation>
                <relation relation="domain_member_category" display="$ctrl.display"
                                                            targetsynsets="$ctrl.targetsynsets"
                                                            relations="$ctrl.synset.relations"
                                                            fullname="Members (category)"></relation>
                <relation relation="domain_member_region" display="$ctrl.display"
                                                          targetsynsets="$ctrl.targetsynsets"
                                                          relations="$ctrl.synset.relations"
                                                          fullname="Members (region)"></relation>
                <relation relation="domain_region" display="$ctrl.display"
                                                   targetsynsets="$ctrl.targetsynsets"
                                                   relations="$ctrl.synset.relations"
                                                   fullname="Domains (region)"></relation>
                <relation relation="domain_topic" display="$ctrl.display"
                                                  targetsynsets="$ctrl.targetsynsets"
                                                  relations="$ctrl.synset.relations"
                                                  fullname="Domains (topic)"></relation>
                <relation relation="entails" display="$ctrl.display"
                                             targetsynsets="$ctrl.targetsynsets"
                                             relations="$ctrl.synset.relations"
                                             fullname="Entails"></relation>
                <relation relation="exemplifies" display="$ctrl.display"
                                                 targetsynsets="$ctrl.targetsynsets"
                                                 relations="$ctrl.synset.relations"
                                                 fullname="Exemplifies"></relation>
                <relation relation="has_domain_region" display="$ctrl.display"
                                                       targetsynsets="$ctrl.targetsynsets"
                                                       relations="$ctrl.synset.relations"
                                                       fullname="Members (region)"></relation>
                <relation relation="has_domain_topic" display="$ctrl.display"
                                                      targetsynsets="$ctrl.targetsynsets"
                                                      relations="$ctrl.synset.relations"
                                                      fullname="Members (topic)"></relation>
                <relation relation="holo_member" display="$ctrl.display"
                                                 targetsynsets="$ctrl.targetsynsets"
                                                 relations="$ctrl.synset.relations"
                                                 fullname="Holonyms (member)"></relation>
                <relation relation="holo_part" display="$ctrl.display"
                                               targetsynsets="$ctrl.targetsynsets"
                                               relations="$ctrl.synset.relations"
                                               fullname="Holonyms (part)"></relation>
                <relation relation="holo_substance" display="$ctrl.display"
                                                    targetsynsets="$ctrl.targetsynsets"
                                                    relations="$ctrl.synset.relations"
                                                    fullname="Holonyms (substance)"></relation>
                <relation relation="also" display="$ctrl.display"
                                          targetsynsets="$ctrl.targetsynsets"
                                          relations="$ctrl.synset.relations"
                                          fullname="See Also"></relation>
                <relation relation="instance_hypernym" display="$ctrl.display"
                                                       targetsynsets="$ctrl.targetsynsets"
                                                       relations="$ctrl.synset.relations"
                                                       fullname="Is Instance of"></relation>
                <relation relation="instance_hyponym" display="$ctrl.display"
                                                      targetsynsets="$ctrl.targetsynsets"
                                                      relations="$ctrl.synset.relations"
                                                      fullname="Instances"></relation>
                <relation relation="is_exemplified_by" display="$ctrl.display"
                                                       targetsynsets="$ctrl.targetsynsets"
                                                       relations="$ctrl.synset.relations"
                                                       fullname="Is Exemplified By"></relation>
                <relation relation="mero_member" display="$ctrl.display"
                                                 targetsynsets="$ctrl.targetsynsets"
                                                 relations="$ctrl.synset.relations"
                                                 fullname="Meronyms (member)"></relation>
                <relation relation="mero_part" display="$ctrl.display"
                                               targetsynsets="$ctrl.targetsynsets"
                                               relations="$ctrl.synset.relations"
                                               fullname="Meronyms (part)"></relation>
                <relation relation="mero_substance" display="$ctrl.display"
                                                    targetsynsets="$ctrl.targetsynsets"
                                                    relations="$ctrl.synset.relations"
                                                    fullname="Meronyms (substance)"></relation>
                <relation relation="participle" display="$ctrl.display"
                                                targetsynsets="$ctrl.targetsynsets"
                                                relations="$ctrl.synset.relations"
                                                fullname="Participles"></relation>
                <relation relation="pertainym" display="$ctrl.display"
                                               targetsynsets="$ctrl.targetsynsets"
                                               relations="$ctrl.synset.relations"
                                               fullname="Pertainyms"></relation>
                <relation relation="similar" display="$ctrl.display"
                                             targetsynsets="$ctrl.targetsynsets"
                                             relations="$ctrl.synset.relations"
                                             fullname="Similar to"></relation>
                <relation relation="verb_group" display="$ctrl.display"
                                                targetsynsets="$ctrl.targetsynsets"
                                                relations="$ctrl.synset.relations"
                                                fullname="Verb groups"></relation>
                <a class="btn" href="#" ng-show="$ctrl.targetsynsetsextra.length > 0"
                ng-click="$ctrl.extendtargetsynsets()">Load More</a>-->
            </div>
            <div class="more">
                <a href="#"
                   v-show="!show_relations"
                   @click="load_targets()">MORE &#x25B6;</a>
            </div>

        </div>
    </div>
</template>
