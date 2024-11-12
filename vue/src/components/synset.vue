<script>

    export default {
        props: ["synset", "display", "focus"],
        data() {
            return {
                show_relations: false,
            }
        }
    }
</script>

<template>
    <div class="synset">
        Synset: {{synset}}<br/>
        Display: {{display}}<br/>
        Focus: {{focus}}<br/>
        <div class="synset-id" v-show="display.ids">
            <span v-bind="synset.id" class="identifier"></span>              (<b class="synset-id-title">Interlingual Index:</b> 
            <span v-bind="synset.ili" class="identifier"></span>)
            <hr/>
        </div>

        <div class="lemmas">
            <span class="pos">({{synset.pos}})</span>
            <span class="lemma" v-for="sense in synset.lemmas">
                <a target="_self" v-bind:href="'/lemma/' + sense.lemma" v-if="sense.lemma != focus">{{sense.lemma}}</a>
                <span class="underline" v-if="sense.lemma == focus">{{sense.lemma}}</span>
                <span v-if="sense.entry_no > 0"><sup>{{sense.entry_no}}</sup></span>
                <span v-if="!display.sensekeys && !display.pronunciation">{{$last ? '' : ','}}</span>
                <span v-if="display.pronunciation && sense.pronunciations.length > 0" class="pronunciation">
                    (Pronunciation:
                    <span v-for="pron in sense.pronunciations">
                        <span v-if="pron.variety" class="pronunciation_variety">({{pron.variety}})</span>
                        {{pron.value}}{{$last ? '' : ', '}}</span>)
                </span><span v-if="!display.sensekeys && display.pronunciation">{{$last ? '' : ','}}</span><span v-bind="sense.sense_key" class="sense_key" v-if="display.sensekeys"></span><span v-if="display.sensekeys">{{$last ? '' : ','}}</span>
            </span>
            <span v-for="rel in synset.relations">
                <span v-if="'rel_type'=='domain_topic'">
                    ((<i><a v-bind:href="'/id/' + rel.target" target="_blank" v-for="r in targetsynsets"><span v-if="id == rel.target">{{r.lemmas[0].lemma}}</span></a></i>))
                </span>
            </span>
            <span ng-bind="synset.definition" class="definition"></span>
            <span v-for="example in synset.examples" class="example"> <span v-if='!example.startsWith("\"")'>&ldquo;</span>{{example}}<span v-if='!example.startsWith("\"")'>&rdquo;</span></span>
            <div v-if="display.topics" class="topic">
                <b>Topic: </b> {{synset.subject}}
            </div>
            <div ng-if="display.subcats" class="subcats">
                <b>Subcategorization Frames:</b>
                <!--<ul ng-repeat="sense in $ctrl.synset.lemmas" ng-if="$ctrl.hasSubcats()">
                    <li class="subcat" ng-repeat="subcat in sense.subcats">{{$ctrl.replaceSubcat(subcat, sense.lemma)}}</span>
                </ul>-->
            </div>
            <div v-if="show_relations" class="relations">
                <relation display="$ctrl.display"
                          targetSynsets="$ctrl.targetsynsets"
                          fullname="Hypernyms"
                          relation="hypernym"
                          relations="$ctrl.synset.relations"></relation>
                <relation relation="hyponym" display="$ctrl.display"
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
                                        ng-click="$ctrl.extendtargetsynsets()">Load More</a>
            </div>

            <div class="more">
                <a href="#"
                   ng-show="!$ctrl.show_relations"
                   ng-click="$ctrl.show_relations = !$ctrl.show_relations">MORE &#x25B6;</a>
            </div>
        </div>
    </div>
</template>
