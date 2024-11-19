<template>
    <v-app>
      <v-main>
          <v-container fill-height style="max-width: 1140px;">
              <v-row>
                  <v-col cols="12" sm="2">
                      <v-row class="d-flex justify-center">
                          <img src="./assets/english.svg" class="oewn_logo"/>
                          <span class="en-title">Open English WordNet</span>
                      </v-row>
                      <v-row style="margin-top:20px;">
                          <v-col xs="1" sm="12" style="padding:0">
                              <RouterLink to="/" class="sidebar_link">Search</RouterLink>
                          </v-col>
                          <v-col xs="1" sm="12" style="padding:0">
                              <RouterLink to="/downloads" class="sidebar_link">Downloads</RouterLink>
                          </v-col>
                          <v-col></v-col>
                      </v-row>
                  </v-col>
                  <v-col cols="12" sm="10">
                      <RouterView/>
                  </v-col>
              </v-row>
              <footer class="footer">
                  <div class="container">
                      <div style="text-align:right;" class="footer1">
                          Open English WordNet is derived from <a href="http://wordnet.princeton.edu/">Princeton WordNet</a> by the Open English WordNet Community and released under the <a href="https://creativecommons.org/licenses/by/4.0/">Creative Commons Attribution (CC-BY) 4.0 License</a>. <a href="https://globalwordnet.github.io/gwadoc/">Further information about WordNet</a>. We welcome any corrections, improvements or other contributions at <a href="http://github.com/globalwordnet/english-wordnet">GitHub</a>. A full list of contributors is available on <a href="https://github.com/globalwordnet/english-wordnet/blob/master/README.md">GitHub</a>.</div>
                      <div class="text-muted footer2" style="text-align:right;"> This interface was created by <a href="http://john.mccr.ae/">John P. McCrae</a> at the <a href="https://dsi.nuigalway.ie/">Data Science Institute</a>, <a href="http://www.universityofgalway.ie">University of Galway</a> (<a href="http://github.com/jmccrae/wordnet-angular">GitHub</a>). Development of this interface is supported by <a href="https://www.sfi.ie/">Science Foundation Ireland</a> as part of the <a href="https://www.insight-centre.org/">Insight Centre for Data Analytics</a> and the European Union's Horizon 2020 research and innovation programme under grant agreement No 731015 (<a href="http://elex.is/">ELEXIS</a>).
                      </div>
                  </div>
              </footer>
            </v-container>
      </v-main>
  </v-app>
</template>

<script>
    export default {
        name: 'App',

        data: () => ({
            //
        }),
    }
</script>
