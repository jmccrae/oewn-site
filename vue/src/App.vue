<template>
  <v-app>
      <v-main>
          <v-container fill-height style="max-width: 1140px;">
              <v-row>
                  <v-col sm="2" cols="12" class="d-flex justify-center align-center">
                      <img src="./assets/english.svg" style="width: 125px;"/>
                  </v-col>
                  <v-col sm="5" cols="12">
                      <h1 class="en-title">Open English WordNet</h1>
                  </v-col>
                  <v-col></v-col>
              </v-row>
              <RouterView/>
              <footer class="footer">
                  <div class="container">
                      <div style="text-align:center;">
                          <button type="button" class="btn download-btn"><a href="/static/english-wordnet-2023.xml.gz" target="_blank">Download as XML</a></button>
                          <button type="button" class="btn download-btn"><a href="/static/english-wordnet-2023.ttl.gz" target="_blank">Download as RDF</a></button>
                          <button type="button" class="btn download-btn"><a href="/static/english-wordnet-2023.zip" target="_blank">Download as WNDB</a></button>
                      </div>
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
