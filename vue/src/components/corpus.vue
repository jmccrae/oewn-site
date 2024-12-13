<script>
    import axios from 'axios';

    export default {
        props: ['synset'],
        data() {
            return {
                corpora: {
                    'raganato_ALL': "Unified WSD Evaluation Corpus",
                    'semcor': "SemCor Corpus",
                    //'wngt': "WordNet Gloss-tagged Corpus",
                    //'omsti': "One-Millions Sense-Tagged Instances"
                },
                data: {
                    'raganato_ALL': [],
                    'semcor': [],
                    //'wngt': [],
                    //'omsti': []
                }
            }
        },
        methods: {
            highlightedStrings(corpus_name) {
                let highlighted = [];
                for (let i = 0; i < this.data[corpus_name].length; i++) {
                    let sentence = this.data[corpus_name][i]["text"];
                    let doc = [];
                    for (const highlight of this.data[corpus_name][i]["highlights"]) {
                        let start = highlight[0];
                        let end = highlight[1];
                        let context_start = Math.max(0, start - 50);
                        let context_end = Math.min(sentence.length, end + 50);
                        let context = {
                            "pre": sentence.substring(context_start, start),
                            "highlight": sentence.substring(start, end),
                            "post": sentence.substring(end, context_end)
                        };
                        doc.push(context);
                    }
                    highlighted.push(doc);
                }
                return highlighted;
            }
        },
        beforeMount() {
            axios.get('/api/corpus/' + this.synset).then(response => {
                console.log(response.data);
                this.data = response.data;
            }).catch(error => {
                console.log(error);
            });
        }
    }
</script>
<template>
    <v-row v-for="(corpus, key) in corpora" :key="key">
        <v-col>
            <h1>{{ corpora[key] }}</h1>
            <v-list>
                <v-list-item v-for="highlighted_doc in highlightedStrings(key)" :key="highlighted">
                    <v-list-item-content>
                        <v-list-item-title>
                            Document
                        </v-list-item-title>
                        <v-list>
                            <v-list-item v-for="highlighted in highlighted_doc" :key="highlighted">
                                <v-list-item-content>
                                    <v-list-item-title>
                                        <v-row>
                                            <v-col cols="5">
                                                <span>{{ highlighted.pre }}</span>
                                            </v-col>
                                            <v-col cols="2">
                                                <span style="background-color: yellow">{{ highlighted.highlight }}</span>
                                            </v-col>
                                            <v-col cols="5">
                                                <span>{{ highlighted.post }}</span>
                                            </v-col>
                                        </v-row>
                                    </v-list-item-title>
                                </v-list-item-content>
                            </v-list-item>
                        </v-list>
                    </v-list-item-content>
                </v-list-item>
            </v-list>
        </v-col>
    </v-row>
</template>
                    
            
