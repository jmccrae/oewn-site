<script>
    const framemap ={
        "nonreferential": "It is ----ing",
        "nonreferential-sent": "It ----s that CLAUSE",
        "ditransitive": "Somebody ----s somebody something",
        "via": "Somebody ----s",
        "via-adj": "Somebody ----s Adjective",
        "via-at": "Somebody ----s at something",
        "via-for": "Somebody ----s for something",
        "via-ger": "Somebody ----s VERB-ing",
        "via-inf": "Somebody ----s INFINITIVE",
        "via-on-anim": "Somebody ----s on somebody",
        "via-on-inanim": "Somebody ----s on something",
        "via-out-of": "Somebody ----s out of somebody",
        "via-pp": "Somebody ----s PP",
        "via-that": "Somebody ----s that CLAUSE",
        "via-to": "Somebody ----s to somebody",
        "via-to-inf": "Somebody ----s to INFINITIVE",
        "via-whether-inf": "Somebody ----s whether INFINITIVE",
        "vibody": "Somebody's (body part) ----s",
        "vii": "Something ----s",
        "vii-adj": "Something ----s Adjective/Noun",
        "vii-inf": "Something ----s INFINITIVE",
        "vii-pp": "Something is ----ing PP",
        "vii-to": "Something ----s to somebody",
        "vtaa": "Somebody ----s somebody",
        "vtaa-inf": "Somebody ----s somebody INFINITIVE",
        "vtaa-into-ger": "Somebody ----s somebody into V-ing something",
        "vtaa-of": "Somebody ----s somebody of something",
        "vtaa-pp": "Somebody ----s somebody PP",
        "vtaa-to-inf": "Somebody ----s somebody to INFINITIVE",
        "vtaa-with": "Somebody ----s somebody with something",
        "vtai": "Somebody ----s something",
        "vtai-from": "Somebody ----s something from somebody",
        "vtai-on": "Somebody ----s something on somebody",
        "vtai-pp": "Somebody ----s something PP",
        "vtai-to": "Somebody ----s something to somebody",
        "vtai-with": "Somebody ----s something with something",
        "vtia": "Something ----s somebody",
        "vtii": "Something ----s something",
        "vtii-adj": "Something ----s something Adjective/Noun",
    }; 

    export default {
        name: "subcat",
        props: [ 'subcats' ],
        methods: {
            thirdPersonForm(word) {
                if (word.endsWith('s')) {
                    return word + "es";
                } else if (word.endsWith('ay') || word.endsWith('ey') || word.endsWith('iy') || word.endsWith('oy') || word.endsWith('uy')) {
                    return word + "s";
                } else if (word.endsWith('y')) {
                    return word.slice(0, -1) + "ies";
                } else if (word.endsWith('e')) {
                    return word + "s";
                } else if (word.endsWith('o')) {
                    return word + "es";
                } else if (word.endsWith('ch')) {
                    return word + "es";
                } else if (word.endsWith('sh')) {
                    return word + "es";
                } else if (word.endsWith('x')) {
                    return word + "es";
                } else {
                    return word + "s";
                }
            },
            gerundForm(word) {
                if (word.endsWith('e')) {
                    return word.slice(0, -1) + "ing";
                } else if (word.endsWith('ie')) {
                    return word.slice(0, -2) + "ying";
                } else {
                    return word + "ing";
                }
            },
            replaceSubcat(subcat, members) {
                subcat = framemap[subcat];
                let mapped_lemmas = [];
                if(subcat.includes('----s')) {
                    for(const member of members) {
                        if (member.includes(' ')) {
                            mapped_lemmas.push(
                                this.thirdPersonForm(member.split(' ')[0]) + " " + member.split(' ').slice(1).join(' '));
                        } else {
                            mapped_lemmas.push(this.thirdPersonForm(member));
                        }
                    }
                    return subcat.replace('----s', mapped_lemmas.join('/'));
                } else if(subcat.includes('----ing')) {
                    for(const member of members) {
                        if (member.includes(' ')) {
                            mapped_lemmas.push(
                                this.gerundForm(member.split(' ')[0]) + " " + member.split(' ').slice(1).join(' '));
                        } else {
                            mapped_lemmas.push(this.gerundForm(member));
                        }
                    }
                    return subcat.replace('----ing', mapped_lemmas.join('/'));
                } else {
                    for(const member of members) {
                        mapped_lemmas.push(member);
                    }
                    return subcat.replace('----', mapped_lemmas.join('/'));
                }
            }
        }
    }
</script>

<template>
    <div class="subcats" v-if="Object.keys(subcats).length > 0">
        <b>Subcategorization frames:</b>
        <ul>
            <li v-for="(members, subcat) in subcats" :key="subcat">{{ replaceSubcat(subcat, members) }}</li>
        </ul>
    </div>
</template>
