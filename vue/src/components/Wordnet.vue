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
                completions: [],
                loading: false,
                searchTerm: "",
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
    <div class="row">
        <div>
            <table width="100%">
                <tr>
                    <td width="100px;" style="vertical-align:top;">
                        <v-btn>{{index}}
                            <v-menu activator="parent">
                                <v-list>
                                    <v-list-item @click="index = 'lemma'">Lemma</v-list-item>
                                    <v-list-item @click="index = 'id'">Identifier</v-list-item>
                                    <v-list-item @click="index = 'ili'">Interlingual Identifier</v-list-item>
                                </v-list>
                            </v-menu>
                        </v-btn>
                    </td>
                    <td class="search-group-parent">
                        <v-autocomplete
                            v-model="query"
                            :items="completions"
                            v-model:search="searchTerm"
                            @change="querySearch"
                            :debounce="300"
                            :loading="loading"
                            auto-select-first
                            ></v-autocomplete>
                    </td>
                </tr>
            </table>
        </div>
        <div>
            <v-btn @click="display_opts = !display_opts"
                class="text-right option_button"
                v-bind:class="{option_button_selected: display_opts}">Options &#x25bc;</v-btn>
        </div>
        <div id="display" class="option_panel" v-if="display_opts" v-transition="expand">
            <div class="option_panel_internal">
                <table>
                    <tr>
                        <td><v-checkbox v-model="display.ids">Show Synset Identifier</v-checkbox></td>
                        <td><v-checkbox v-model="display.sensekeys">Show Sense Keys</v-checkbox></td>
                        <td><v-checkbox v-model="display.subcats">Show Subcategorization Frames</v-checkbox></td>
                        <td><v-checkbox v-model="display.topics">Show Topics</v-checkbox></td>
                        <td><v-checkbox v-model="display.pronunciation">Show Pronunciation</v-checkbox></td>
                    </tr>
                </table>
            </div>
        </div>
        <span class="pos_grp" v-if="Object.values(synsets).some(ss => ss.partOfSpeech == 'n')">
            <h3 class="pos_label">Nouns</h3>
            <div v-for="ss in synsets">
                <synset v-if="ss.partOfSpeech == 'n'"
                :synset="ss" :display="display" :focus="focus" :entries="entries"></synset>
            </div>
        </span>
        <span class="pos_grp" v-if="Object.values(synsets).some(ss => ss.partOfSpeech == 'v')">
            <h3 class="pos_label">Verbs</h3>
            <div v-for="ss in synsets">
                <synset v-if="ss.partOfSpeech == 'v'"
                :synset="ss" :display="display" :focus="focus" :entries="entries"></synset>
            </div>
        </span>
        <span class="pos_grp" v-if="Object.values(synsets).some(ss => ss.partOfSpeech == 'r')">
            <h3 class="pos_label">Adverbs</h3>
            <div v-for="ss in synsets">
                <synset v-if="ss.partOfSpeech == 'r'"
                :synset="ss" :display="display" :focus="focus" :entries="entries"></synset>
            </div>
        </span>
        <span class="pos_grp" v-if="Object.values(synsets).some(ss => ss.partOfSpeech == 'a' || ss.partOfSpeech == 's')">
            <h3 class="pos_label">Adjectives</h3>
            <div v-for="ss in synsets">
                <synset v-if="ss.partOfSpeech == 'a' || ss.partOfSpeech == 's'"
                :synset="ss" :display="display" :focus="focus" :entries="entries"></synset>
            </div>
        </span>
        <div class="text-right" ng-show="link">
            <b>Download As:</b>&nbsp;&nbsp;<a target="_self" ng-href="/json/{{link}}">JSON</a>&nbsp;&nbsp;
            <a target="_self" ng-href="/ttl/{{link}}">RDF</a>&nbsp;&nbsp;
            <a target="_self" ng-href="/xml/{{link}}">XML</a>
        </div>
    </div>
</template>
