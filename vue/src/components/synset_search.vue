<script>
    import axios from 'axios';

    export default {
        props: {
            "value": {
                type: String
            },
            "lemma": {
                type: String
            },
            "label": {
                type: String
            }
        },
        name: 'synsetSearch',
        data() {
            return {
                query: '',
                lastQuery: '',
                searchTerm: '',
                loading: false,
                completions: [],
                definition: '',
                query_lemma: ''
            }
        },
        methods: {
            autocomplete() {
                if(this.searchTerm == null || this.searchTerm == "") {
                    return;
                }
                this.loading = true;
                // if ' - ' is in the searchTerm only use the text to the left of the search
                // term to search for completions
                let searchTerm = this.searchTerm;
                if(searchTerm.includes(" - ")) {
                    searchTerm = searchTerm.split(" - ")[0];
                }
                axios.
                    get("/autocomplete_synset/" + searchTerm)
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
            search() {
                for (const completion of this.completions) {
                    if (completion.value == this.query) {
                        this.definition = completion.definition;
                        this.query_lemma = completion.lemma;
                        this.$emit('update:value', completion.value);
                        this.$emit('update:lemma', this.query_lemma);
                        this.$emit('change_value', completion.value, this.query_lemma);
                        return;
                    }
                };
            }
        },
        watch: {
            searchTerm(newVal) {
                this.autocomplete();
            }
        },
        beforeMount() {
            if (this.value) {
                axios
                    .get("/json/id/" + this.value)
                    .then(response => {
                        if (response.data.synsets.length == 0) {
                            return;
                        }
                        if(this.lemma) {
                            this.query_lemma = this.lemma;
                        } else {
                            this.query_lemma = response.data.synsets[0].members[0].lemma;
                        }
                        this.query = {
                            "title": response.data.synsets[0].members[0].lemma + " - " + response.data.synsets[0].definition[0],
                            "value": response.data.synsets[0].id
                        };
                        this.definition = response.data.synsets[0].definition[0];
                    })
                    .catch(error => {
                        console.log(error);
                    });
            }
        },

    }
</script>

<template>
    <v-autocomplete
            :label="label"
            v-model="query"
            :items="completions"
            :debounce="300"
            :loading="loading"
            v-model:search="searchTerm"
            auto-select-first
            item-title="title"
            item-value="value"
            @change="search"
            class="ma-0 pa-0"></v-autocomplete>
</template>

