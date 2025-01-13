<script>
    import axios from 'axios';
    import yaml from 'yaml';
    import synsetSearch from './synset_search.vue';

    const RELATIONS = {
        "agent": { "sense": true, "name": "Agent" },
        "also": { "sense": false, "name": "See Also" },
        "antonym": { "sense": true, "name": "Antonym" },
        "attribute": { "sense": false, "name": "Attribute" },
        "body_part": { "sense": true, "name": "Body Part" },
        "by_means_of": { "sense": true, "name": "By Means Of" },
        "causes": { "sense": false, "name": "Causes" },
        "derivation": { "sense": true, "name": "Derivation" },
        "destination": { "sense": true, "name": "Destination" },
        "domain_region": { "sense": false, "name": "Domain (Region)" },
        "domain_topic": { "sense": false, "name": "Domain (Topic)" },
        "entails": { "sense": false, "name": "Entails" },
        "event": { "sense": true, "name": "Event" },
        "exemplifies": { "sense": false, "name": "Exemplifies" },
        "exemplifies_sense": { "sense": true, "name": "Exemplifies (Sense)" },
        "has_domain_region": { "sense": false, "name": "Has Domain (Region)" },
        "has_domain_topic": { "sense": false, "name": "Has Domain (Topic)" },
        "holo_member": { "sense": false, "name": "Holonym (Member)" },
        "holo_part": { "sense": false, "name": "Holonym (Part)" },
        "holo_substance": { "sense": false, "name": "Holonym (Substance)" },
        "hypernym": { "sense": false, "name": "Hypernym" },
        "hyponym": { "sense": false, "name": "Hyponym" },
        "instance_hypernym": { "sense": false, "name": "Instance Hypernym" },
        "instance_hyponym": { "sense": false, "name": "Instance Hyponym" },
        "instrument": { "sense": true, "name": "Instrument" },
        "is_agent_of": { "sense": true, "name": "Is Agent Of" },
        "is_body_part_of": { "sense": true, "name": "Is Body Part Of" },
        "is_by_means_of": { "sense": true, "name": "Is By Means Of" },
        "is_caused_by": { "sense": false, "name": "Is Caused By" },
        "is_destination_of": { "sense": true, "name": "Is Destination Of" },
        "is_entailed_by": { "sense": false, "name": "Is Entailed By" },
        "is_event_of": { "sense": true, "name": "Is Event Of" },
        "is_exemplified_by": { "sense": false, "name": "Is Exemplified By" },
        "is_exemplified_by_sense": { "sense": true, "name": "Is Exemplified By (Sense)" },
        "is_instrument_of": { "sense": true, "name": "Is Instrument Of" },
        "is_location_of": { "sense": true, "name": "Is Location Of" },
        "is_material_of": { "sense": true, "name": "Is Material Of" },
        "is_participle_of": { "sense": true, "name": "Is Participle Of" },
        "is_pertainym_of": { "sense": true, "name": "Is Pertainym Of" },
        "is_property_of": { "sense": true, "name": "Is Property Of" },
        "is_result_of": { "sense": true, "name": "Is Result Of" },
        "is_state_of": { "sense": true, "name": "Is State Of" },
        "is_undergoer_of": { "sense": true, "name": "Is Undergoer Of" },
        "is_used_by": { "sense": true, "name": "Is Used By" },
        "is_vehicle_of": { "sense": true, "name": "Is Vehicle Of" },
        "location": { "sense": true, "name": "Location" },
        "material": { "sense": true, "name": "Material" },
        "mero_member": { "sense": false, "name": "Meronym (Member)" },
        "mero_part": { "sense": false, "name": "Meronym (Part)" },
        "mero_substance": { "sense": false, "name": "Meronym (Substance)" },
        "participle": { "sense": true, "name": "Participle" },
        "pertainym": { "sense": true, "name": "Pertainym" },
        "property": { "sense": true, "name": "Property" },
        "result": { "sense": true, "name": "Result" },
        "similar": { "sense": false, "name": "Similar" },
        "state": { "sense": true, "name": "State" },
        "undergoer": { "sense": true, "name": "Undergoer" },
        "uses": { "sense": true, "name": "Uses" },
        "vehicle": { "sense": true, "name": "Vehicle" },
    };


    export default {
        name: 'editor',
        data() {
            return {
                synset: {},
                relations: {},
                old_relations: {},
                query: '',
                ssid: '',
                lastQuery: '',
                completions: [],
                loading: false,
                searchTerm: '',
                relation_names: [],
                changes: [],
                yaml_changes: "",
                addMemberDialog: false,
                addMemberValid: false,
                newMember: "",
                addExampleDialog: false,
                newExample: "",
                deleteSynsetDialog: false,
                deleteReason: "",
                deleteSynset: "",
                deleteSynsetValid: false,
                addSynsetDialog: false,
                addSynsetValid: false,
                newSynset: {
                    "definition": "",
                    "lexfile": "",
                    "lemma": ""
                },
                lexfiles: [
                    "adj.all",
                    "adj.pert",
                    "adv.all",
                    "noun.Tops",
                    "noun.act",
                    "noun.animal",
                    "noun.artifact",
                    "noun.attribute",
                    "noun.body",
                    "noun.cognition",
                    "noun.communication",
                    "noun.event",
                    "noun.feeling",
                    "noun.food",
                    "noun.group",
                    "noun.location",
                    "noun.motive",
                    "noun.object",
                    "noun.person",
                    "noun.phenomenon",
                    "noun.plant",
                    "noun.possession",
                    "noun.process",
                    "noun.quantity",
                    "noun.relation",
                    "noun.shape",
                    "noun.state",
                    "noun.substance",
                    "noun.time",
                    "verb.body",
                    "verb.change",
                    "verb.cognition",
                    "verb.communication",
                    "verb.competition",
                    "verb.consumption",
                    "verb.contact",
                    "verb.creation",
                    "verb.emotion",
                    "verb.motion",
                    "verb.perception",
                    "verb.possession",
                    "verb.social",
                    "verb.stative",
                    "verb.weather"
                ]
            }
        },
        methods: {
            querySearch() {
                if(this.query == null || this.query == "" || this.query === this.lastQuery) {
                    return;
                }
                const query = this.query;
                axios
                    .get('/json/id/' + query)
                    .then(response => {
                        this.synset = response.data.synsets[0];
                        this.$router.push("/edit/" + query);
                        this.lastQuery = query;
                    })
                    .catch(error => {
                        console.log(error);
                    });
            },
            autocomplete() {
                this.loading = true;
                axios.
                    get("/autocomplete_synset/" + this.searchTerm)
                    .then(response => {
                        this.completions = response.data;
                    })
                    .catch(error => {
                        console.log(error);
                    })
                    .finally(() => {
                        this.loading = false;
                    });
            },
            changeDefinition() {
                this.changes = this.changes.filter(change => !('change_definition' in change) || change.change_definition.synset !== this.synset.id);
                this.changes.push({"change_definition": {"synset": this.synset.id, "definition": this.synset.definition[0]}});
            },
            changeILI() {
                this.changes = this.changes.filter(change => !('change_ili' in change) || change.change_ili.synset !== this.synset.id);
                this.changes.push({"change_ili": {"synset": this.synset.id, "ili": this.synset.ili}});
            },
            changeWikiData() {
                this.changes = this.changes.filter(change => !('change_wikidata' in change) || change.change_wikidata.synset !== this.synset.id);
                this.changes.push({"change_wikidata": {"synset": this.synset.id, "wikidata": this.synset.wikidata}});
            },
            changeSource() {
                this.changes = this.changes.filter(change => !('change_source' in change) || change.change_source.synset !== this.synset.id);
                this.changes.push({"change_source": {"synset": this.synset.id, "source": this.synset.source}});
            },
            changeMembers() {
                this.changes = this.changes.filter(change => !('change_members' in change) || change.change_members.synset !== this.synset.id);
                var member_list = [];
                for (const member of this.synset.members) {
                    member_list.push(member.lemma);
                }
                this.changes.push({"change_members": {"synset": this.synset.id, "members": member_list}});
            },
            addMember() {
                this.addMemberDialog = false; 
                this.synset.members.push({'lemma': this.newMember});
                this.newMember = "";
                this.changeMembers();
            },
            moveMember(index) {
                alert("TODO");
            },
            upMember(index) {
                this.synset.members.splice(index - 1, 0, this.synset.members.splice(index, 1)[0]);
                this.changeMembers();
            },
            downMember(index) {
                this.synset.members.splice(index + 1, 0, this.synset.members.splice(index, 1)[0]);
                this.changeMembers();
            },
            removeMember(index) {
                this.synset.members.splice(index, 1);
                this.changeMembers();
            },
            changeRelationSynset(index, value, lemma) {
                var obj = {"delete_relation": { "source": this.synset.id,
                    "target": this.old_relations[index].target_synset }};
                if("source_lemma" in this.relations[index]) {
                    obj.delete_relation.source_lemma = this.old_relations[index].source_lemma;
                }
                if("target_lemma" in this.relations[index]) {
                    obj.delete_relation.target_lemma = this.old_relations[index].target_lemma;
                }
                this.changes.push(obj);

                this.relations[index].target_synset = value;
                if ("target_lemma" in this.relations[index]) {
                    this.relations[index].target_lemma = lemma;
                }

                var obj = {"add_relation": { "source": this.synset.id, 
                    "relation": this.relations[index].rel, 
                    "target": value }};
                if("target_lemma" in this.relations[index]) {
                    obj.change_relation.target_lemma = lemma;
                }
                if("source_lemma" in this.relations[index]) {
                    obj.change_relation.source_lemma = this.relations[index].source_lemma;
                }
                this.changes.push(obj);
                this.yaml_changes = yaml.stringify(this.changes);
            },
            is_sense(rel) {
                return RELATIONS[rel].sense;
            },
            addRelation() {
                this.relations.push({"rel": "hypernym", "target_synset": "00001740-n"});
                this.changes.push({"add_relation": {"source": this.synset.id, "relation": "hypernym", "target": "00001740-n"}});
                this.yaml_changes = yaml.stringify(this.changes);
            },
            deleteRelation(index) {
                var obj = {"delete_relation": {"source": this.synset.id, "target": this.old_relations[index].target_synset}};
                if("source_lemma" in this.old_relations[index]) {
                    obj.delete_relation.source_lemma = this.old_relations[index].source_lemma;
                }
                if("target_lemma" in this.old_relations[index]) {
                    obj.delete_relation.target_lemma = this.old_relations[index].target_lemma;
                }
                this.changes.push(obj);
                this.relations.splice(index, 1);
                this.yaml_changes = yaml.stringify(this.changes);
            },
            changeRelationType(index) {
                var obj = {"delete_relation": { "source": this.synset.id,
                    "target": this.old_relations[index].target_synset }};
                if("source_lemma" in this.old_relations[index]) {
                    obj.delete_relation.source_lemma = this.old_relations[index].source_lemma;
                }
                if("target_lemma" in this.old_relations[index]) {
                    obj.delete_relation.target_lemma = this.old_relations[index].target_lemma;
                }
                this.changes.push(obj);
                var obj = {"add_relation": { "source": this.synset.id, 
                    "relation": this.relations[index].rel, 
                    "target": this.relations[index].target_synset }};
                if("source_lemma" in this.relations[index]) {
                    obj.add_relation.source_lemma = this.relations[index].source_lemma;
                }
                if("target_lemma" in this.relations[index]) {
                    obj.add_relation.target_lemma = this.relations[index].target_lemma;
                }
                this.changes.push(obj);
                this.yaml_changes = yaml.stringify(this.changes);
            },
            addExample() {
                this.addExampleDialog = false; 
                this.synset.example.push(this.newExample);
                this.changes.push({"add_example": {"synset": this.synset.id, "example": this.newExample}});
                this.yaml_changes = yaml.stringify(this.changes);
                this.newExample = "";
            },
            removeExample(index) {
                this.synset.example.splice(index, 1);
                this.changes.push({"delete_example": {"synset": this.synset.id, "number": index}});
                this.yaml_changes = yaml.stringify(this.changes);
            },
            deleteSynset() {
                this.deleteSynsetDialog = false;
                this.changes.push({"delete_synset": {"synset": this.synset.id, 
                    "reason": this.deleteReason,
                    "superseded_by": this.deleteSynset}});
                this.yaml_changes = yaml.stringify(this.changes);
                this.$router.push("/edit");
                this.synset = {};
            },
            changeDeleteSynset(value) {
                this.deleteSynset = value;
            },
        },
        watch: {
            searchTerm(val) {
                this.autocomplete();
            },
            query(newVal) {
                this.querySearch();
            },
            synset(newVal) {
                this.relations = [];
                for (const rel_name of Object.keys(RELATIONS)) {
                    if (newVal[rel_name]) {
                        for (const relVal of newVal[rel_name]) {
                            if(RELATIONS[rel_name].sense) {
                                relVal["rel"] = rel_name;
                                this.relations.push(relVal);
                            } else {
                                this.relations.push({ "rel": rel_name, "target_synset": relVal });
                            }
                        }
                    }
                }
                this.old_relations = JSON.parse(JSON.stringify(this.relations));
            },
            changes(newVal) {
                this.yaml_changes = yaml.stringify(newVal);
            }
        },
        beforeMount() {
            if (this.$route.params.query) {
                this.query = this.$route.params.query;
                this.querySearch();
            }
            for (const rel_name of Object.keys(RELATIONS)) {
                this.relation_names.push({
                    "title": RELATIONS[rel_name].name,
                    "value": rel_name
                });
            }
        },
        components: {
            synsetSearch
        },
    }
</script>

<template>
    <v-row>
        <v-col>
            <h1>English WordNet Editor - {{ synset.id }}</h1>
        </v-col>
    </v-row>
    <v-row>
        <v-col cols="8">
            <v-autocomplete
                    v-model="query"
                    :items="completions"
                    v-model:search="searchTerm"
                    @change="querySearch"
                    :debounce="300"
                    :loading="loading"
                    auto-select-first></v-autocomplete>
        </v-col>
        <v-col cols="4">
            <v-btn @click="deleteSynsetDialog = true"
               :disabled="Object.keys(synset).length == 0">
                <v-icon>mdi-delete</v-icon> Delete Synset
            </v-btn>
            <v-dialog v-model="deleteSynsetDialog" max-width="400px">
                <v-card>
                    <v-card-title>Delete synset</v-card-title>
                    <v-card-text>
                        <v-form v-model="deleteSynsetValid" fail-fast>
                            <v-text-field label="Reason" v-model="deleteReason"
                                required :rules="[v => !!v || 'Reason is required']"></v-text-field>
                            <synsetSearch
                                    @change_value="(value, lemma) => changeDeleteSynset(value)"
                                    label="Superseded by" :value="deleteSynset"
                                    :rules="[deleteSynset != '' || 'Superseded by is required']"
                                         ></synsetSearch>
                        </v-form>
                    </v-card-text>
                    <v-card-actions>
                        <v-btn @click="deleteSynsetDialog = false">Cancel</v-btn>
                        <v-btn @click="deleteSynset()" :disabled="!deleteSynsetValid">Delete</v-btn>
                    </v-card-actions>
                </v-card>
            </v-dialog>
            <v-btn @click="addSynsetDialog = true">
                <v-icon>mdi-plus</v-icon> Add Synset
            </v-btn>
            <v-dialog v-model="addSynsetDialog" max-width="400px">
                <v-card>
                    <v-card-title>Add synset</v-card-title>
                    <v-card-text>
                        <v-form v-model="addSynsetValid" fast-fail>
                            <v-text-field label="Definition" 
                                          v-model="newSynset.definition" required
                                          :rules="[v => !!v || 'Definition is required']"></v-text-field>
                            <v-select label="Lexicographer File" :items="lexfiles" 
                                      v-model="newSynset.lexfile" required 
                                      :rules="[v => !!v || 'Lexicographer file is required']"></v-select>
                            <v-text-field label="Lemma" v-model="newSynset.lemma" 
                                                        required
                                                        :rules="[v => !!v || 'Lemma is required']"></v-text-field>
                        </v-form>
                    </v-card-text>
                    <v-card-actions>
                        <v-btn @click="addSynsetDialog = false">Cancel</v-btn>
                        <v-btn @click="addSynset()" :disabled="!addSynsetValid">Add</v-btn>
                    </v-card-actions>
                </v-card>
            </v-dialog>
        </v-col>
    </v-row>
    <v-row v-if="Object.keys(synset).length > 0">
        <v-col>
            <v-form>
                <v-row>
                    <v-text-field label="Definition" v-model="synset.definition[0]" v-if="synset.definition"
                        @change="changeDefinition()" required></v-text-field>
                </v-row>
                <v-row>
                    <v-col cols="4">
                        <v-text-field label="ILI" v-model="synset.ili"
                            @change="changeILI()"></v-text-field>
                    </v-col>
                    <v-col cols="4">
                        <v-text-field label="Wikidata" v-model="synset.wikidata"
                            @change="changeWikidata()"></v-text-field>
                    </v-col>
                    <v-col cols="4">
                        <v-text-field label="Source" v-model="synset.source"
                            @change="changeSource()"></v-text-field>
                    </v-col>
                </v-row>
                <v-row>
                    <v-col cols="11">
                        <h3>Members</h3>
                    </v-col>
                    <v-col cols="1">
                        <v-btn icon @click="addMemberDialog = true">
                            <v-icon>mdi-plus</v-icon>
                        </v-btn>
                        <v-dialog v-model="addMemberDialog" max-width="400px">
                                <v-card>
                                    <v-card-title>Add Member</v-card-title>
                                    <v-card-text>
                                        <v-form v-model="addMemberValid">
                                            <v-text-field v-model="newMember" required></v-text-field>
                                        </v-form>
                                    </v-card-text>
                                    <v-card-actions>
                                        <v-btn @click="addMemberDialog = false">Cancel</v-btn>
                                        <v-btn @click="addMember()"
                                            :disabled="!addMemberValid">Add</v-btn>
                                    </v-card-actions>
                                </v-card>
                        </v-dialog>
                    </v-col>
                </v-row>
                <v-row>
                    <v-col cols="12">
                        <v-card max-width="400px">
                            <v-list dense>
                                <v-list-item v-for="(member, index) in synset.members" :key="index">
                                    <v-list-item-title>{{ member.lemma }}</v-list-item-title>
                                    <template v-slot:append>
                                        <v-btn icon @click="upMember(index)" class="mr-2" v-if="index > 0"><v-icon>mdi-arrow-up-bold</v-icon></v-btn>
                                        <div class="mr-14" v-if="index == 0"></div>
                                        <v-btn icon @click="downMember(index)" class="mr-2" v-if="index < synset.members.length - 1"><v-icon>mdi-arrow-down-bold</v-icon></v-btn>
                                        <div class="mr-14" v-if="index == synset.members.length - 1"></div>
                                        <v-btn icon @click="moveMember(index)" class="mr-2"><v-icon>mdi-swap-horizontal-bold</v-icon></v-btn>
                                        <v-btn icon @click="removeMember(index)">
                                            <v-icon>mdi-delete</v-icon>
                                        </v-btn>
                                    </template>
                                </v-list-item>
                            </v-list>
                        </v-card>
                    </v-col>
                </v-row>
                <v-row>
                    <v-col cols="11">
                        <h3>Relations</h3>
                    </v-col>
                    <v-col cols="1">
                        <v-btn icon @click="addRelation()">
                            <v-icon>mdi-plus</v-icon>
                        </v-btn>
                    </v-col>
                </v-row>
                <v-row v-for="(relation, index) in relations" class="ma-0 pa-0" :key="index">
                    <v-col cols="2" class="pa-0">
                        <v-select
                                :items="synset.members.map(member => member.lemma)"
                                v-model="relation.source_lemma"
                                v-if="is_sense(relation.rel)"
                                @update:modelValue="changeRelationType(index)"></v-select>
                    </v-col>
                    <v-col cols="3" class="pa-0">
                        <v-select :items="relation_names" v-model="relation.rel"
                            @update:modelValue="changeRelationType(index)"></v-select>
                    </v-col>
                    <v-col cols="6" class="pa-0">
                        <synsetSearch 
                            @change_value="(value, lemma) => changeRelationSynset(index, value, lemma)"
                            :value="relation.target_synset"
                            :lemma="relation.target_lemma"></synsetSearch>
                    </v-col>
                    <v-col cols="1">
                        <v-btn icon @click="deleteRelation(index)">
                            <v-icon>mdi-delete</v-icon>
                        </v-btn>
                    </v-col>
                </v-row>
                <v-row>
                    <v-col cols="11">
                        <h3>Examples</h3>
                    </v-col>
                    <v-col cols="1">
                        <v-btn icon @click="addExampleDialog = true">
                            <v-icon>mdi-plus</v-icon>
                        </v-btn>
                        <v-dialog v-model="addExampleDialog" max-width="400px">
                            <v-card>
                                <v-card-title>Add Example</v-card-title>
                                <v-card-text>
                                    <v-text-field v-model="newExample"></v-text-field>
                                </v-card-text>
                                <v-card-actions>
                                    <v-btn @click="addExampleDialog = false">Cancel</v-btn>
                                    <v-btn @click="addExample()">Add</v-btn>
                                </v-card-actions>
                            </v-card>
                        </v-dialog>
                    </v-col>
                </v-row>
                <v-row>
                    <v-col cols="12">
                        <v-card max-width="600px">
                            <v-list dense>
                                <v-list-item v-for="(example, index) in synset.example" :key="index">
                                    <v-list-item-title>{{ example }}</v-list-item-title>
                                    <template v-slot:append>
                                        <v-btn icon @click="removeExample(index)">
                                            <v-icon>mdi-delete</v-icon>
                                        </v-btn>
                                    </template>
                                </v-list-item>
                            </v-list>
                        </v-card>
                    </v-col>
                </v-row>
             </v-form>
        </v-col>
    </v-row>
    <v-row>
        <v-col cols="3">
            <b>Number of changes:</b> {{ changes.length }}
        </v-col>
        <v-col cols="6">
        </v-col>
        <v-col cols="3">
            <v-btn v-bind:href="'data:text/yaml;charset=utf-8,' + encodeURIComponent(yaml_changes)" download="changes.yaml" :disabled="changes.length == 0">
                <v-icon>mdi-download</v-icon>Download Changes</v-btn>
        </v-col>
    </v-row>
</template>
