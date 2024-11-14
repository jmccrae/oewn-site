<script>
    import axios from "axios";
    import synset from "./synset.vue";

    export default {
        data() {
            return {
                index: 'lemma',
                query: '',
                lastQuery: '',
                results: [],
                query_cleared: false,
                display_opts: false,
                display: {
                    ids: false,
                    sensekeys: false,
                    subcats: false,
                    topics: false,
                    pronunciation: false
                },
                synsets: {},
                entries: {},
                target_labels: {},
                completions: [],
                loading: false,
                searchTerm: "",
                opt_props: false,
            }
        },
        methods: {
            querySearch() {
                if(this.query == null || this.query == "" || this.query === this.lastQuery) {
                    return;
                }
                const query = this.query;
                axios
                    .get('/json/' + this.index + '/' + query)
                    .then(response => {
                        this.synsets = response.data.synsets;
                        this.entries = response.data.entries;
                        this.target_labels = response.data.target_labels;
                        this.$router.push("/" + this.index + "/" + query);
                        this.lastQuery = query;
                    })
                    .catch(error => {
                        console.log(error);
                    });
            },
            autocomplete() {
                if (this.searchTerm < 3) {
                    return;
                }
                this.loading = true;
                axios
                    .get('/autocomplete/' + this.index + '/' + this.searchTerm)
                    .then(response => {
                        this.completions = response.data;
                    })
                    .catch(error => {
                        console.log(error);
                    })
                    .finally(() => {
                        this.loading = false;
                    });
            }

        },
        watch: {
            searchTerm(val) {
                this.autocomplete();
            },
            query(newVal) {
                this.querySearch();
            }
        },
        components: {
            synset
        },
        beforeMount() {
            if (this.$route.params.index && this.$route.params.query) {
                this.index = this.$route.params.index;
                this.query = this.$route.params.query;
                this.querySearch();
            }
        }
    }
</script>

<template>
    <v-row>
        <v-col sm="1" cols="12" class="d-flex justify-center align-center">
            <v-btn variant="text">{{index}}
                <v-menu activator="parent">
                    <v-list>
                        <v-list-item @click="index = 'lemma'">Lemma</v-list-item>
                        <v-list-item @click="index = 'id'">Identifier</v-list-item>
                        <v-list-item @click="index = 'ili'">Interlingual Identifier</v-list-item>
                    </v-list>
                </v-menu>
            </v-btn>
        </v-col>
        <v-col class="d-flex justify-center align-center">
            <v-autocomplete
                v-model="query"
                :items="completions"
                v-model:search="searchTerm"
                @change="querySearch"
                :debounce="300"
                :loading="loading"
                auto-select-first
                ></v-autocomplete>
        </v-col>
        <v-col sm="1" cols="12" class="d-flex justify-center align-center">
            <v-btn
                id="opts-activator"
                variant="text">Options</v-btn>
            <v-menu activator="#opts-activator"
                :close-on-content-click="false"
                location="bottom end">
                <v-card min-width="300">
                    <v-list density="compact">
                        <v-list-item>
                            <v-switch v-model="display.ids" label="Synset Identifiers"></v-switch>
                        </v-list-item>
                    </v-list>
                    <v-list>
                        <v-list-item>
                            <v-switch v-model="display.sensekeys" label="Sense Keys"></v-switch>
                        </v-list-item>
                    </v-list>
                    <v-list>
                        <v-list-item>
                            <v-switch v-model="display.subcats" label="Subcategorization Frames"></v-switch>
                        </v-list-item>
                    </v-list>
                    <v-list>
                        <v-list-item>
                            <v-switch v-model="display.topics" label="Topics"></v-switch>
                        </v-list-item>
                    </v-list>
                    <v-list>
                        <v-list-item>
                            <v-switch v-model="display.pronunciation" label="Pronunciation"></v-switch>
                        </v-list-item>
                    </v-list>
                </v-card>
            </v-menu>
        </v-col>
    </v-row>
    <v-row>
        <v-col>
            <span class="pos_grp" v-if="Object.values(synsets).some(ss => ss.partOfSpeech == 'n')">
                <h3 class="pos_label">Nouns</h3>
                <div v-for="ss in synsets">
                    <synset v-if="ss.partOfSpeech == 'n'"
                    :synset="ss" :display="display" :focus="focus" :entries="entries"
                    :target_labels="target_labels"></synset>
                </div>
            </span>
            <span class="pos_grp" v-if="Object.values(synsets).some(ss => ss.partOfSpeech == 'v')">
                <h3 class="pos_label">Verbs</h3>
                <div v-for="ss in synsets">
                    <synset v-if="ss.partOfSpeech == 'v'"
                    :synset="ss" :display="display" :focus="focus" :entries="entries"
                    :target_labels="target_labels"></synset>
                </div>
            </span>
            <span class="pos_grp" v-if="Object.values(synsets).some(ss => ss.partOfSpeech == 'r')">
                <h3 class="pos_label">Adverbs</h3>
                <div v-for="ss in synsets">
                    <synset v-if="ss.partOfSpeech == 'r'"
                    :synset="ss" :display="display" :focus="focus" :entries="entries"
                    :target_labels="target_labels"></synset>
                </div>
            </span>
            <span class="pos_grp" v-if="Object.values(synsets).some(ss => ss.partOfSpeech == 'a' || ss.partOfSpeech == 's')">
                <h3 class="pos_label">Adjectives</h3>
                <div v-for="ss in synsets">
                    <synset v-if="ss.partOfSpeech == 'a' || ss.partOfSpeech == 's'"
                    :synset="ss" :display="display" :focus="focus" :entries="entries"
                    :target_labels="target_labels"></synset>
                </div>
            </span>
        </v-col>
    </v-row>
    <v-row>
        <v-col sm="9" cols="12"></v-col>
        <v-col sm="3" cols="12">
            <div class="text-right" ng-show="link">
                <b>Download As:</b>&nbsp;&nbsp;<a target="_self" ng-href="/json/{{link}}">JSON</a>&nbsp;&nbsp;
                <a target="_self" ng-href="/ttl/{{link}}">RDF</a>&nbsp;&nbsp;
                <a target="_self" ng-href="/xml/{{link}}">XML</a>
            </div>
        </v-col>
    </v-row>
</template>
