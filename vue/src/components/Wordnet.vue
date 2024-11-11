<script>
  export default {
    data() {
      return {
        index: 'lemma',
        query: '',
        results: []
      }
    },
    methods: {
      querySearch() {
        console.log('querySearch')
      },
      keyPress(keyCode) {
        console.log('keyPress', keyCode)
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
                        <v-menu>
                            <template v-slot:activator="{ on }">
                                <v-btn v-on="on">{{ index }}</v-btn>
                            </template>
                            <v-list>
                                <v-list-item @click="index = 'lemma'">Lemma</v-list-item>
                                <v-list-item @click="index = 'id'">Identifier</v-list-item>
                                <v-list-item @click="index = 'ili'">Interlingual Identifier</v-list-item>
                            </v-list>
                        </v-menu>
                    </td>
                    <td class="search-group-parent">
                        <input type="text" class="search-control" placeholder="Search"
                                                                  ng-model="ctrl.query" ng-change="ctrl.querySearch()"
                                                                                        ng-keyup="ctrl.keyPress($event.keyCode)">
                        <ul class="list-group search-group" ng-if="ctrl.results && ctrl.results.length > 0">
                            <li ng-repeat="result in ctrl.results" class="list-group-item" ng-click="ctrl.selectedItemChange(result)">
                                <a ng-bind="result.display"></a>
                            </li>
                        </ul>
                        <ul class="list-group search-group" ng-if="ctrl.results && ctrl.results.length == 0 && ctrl.query.length > 0 && !ctrl.query_cleared">
                            <li class="list-group-item"><i>No results</i></li>
                        </ul>
                    </td>
                </tr>
            </table>
        </div>
        <div>
            <md-button slide-toggle="#display" class="pull-right option_button" ng-click="display.display = !display.display"
                                                                                ng-class="{option_button_selected: display.display}">Options  &#x25bc;</md-button>
        </div>
        <div id="display" class="slideable option_panel">
            <div class="option_panel_internal">
                <table>
                    <tr>
                        <td><md-checkbox ng-model="display.ids">Show Synset Identifier</md-checkbox></td>
                        <td><md-checkbox ng-model="display.sensekeys">Show Sense Keys</md-checkbox></td>
                        <td><md-checkbox ng-model="display.subcats">Show Subcategorization Frames</md-checkbox></td>
                        <td><md-checkbox ng-model="display.topics">Show Topics</md-checkbox></td>
                        <td><md-checkbox ng-model="display.pronunciation">Show Pronunciation</md-checkbox></td>
                    </tr>
                </table>
            </div>
        </div>
        <span class="pos_grp">
            <h3 class="pos_label">Nouns</h3>
            <div ng-repeat="synset in synsets | filter:{'pos':'n'}">
                <synset synset="synset" display="display" focus="focus"></synset>
            </div>
        </span>
        <span class="pos_grp">
            <h3 class="pos_label">Verbs</h3>
            <div ng-repeat="synset in synsets | filter:{'pos':'v'}">
                <synset synset="synset" display="display" focus="focus"></synset>
            </div>
        </span>
        <span class="pos_grp">
            <h3 class="pos_label">Adverbs</h3>
            <div ng-repeat="synset in synsets | filter:{'pos':'r'}">
                <synset synset="synset" display="display" focus="focus"></synset>
            </div>
        </span>
        <span class="pos_grp">
            <h3 class="pos_label">Adjectives</h3>
            <div ng-repeat="synset in synsets | isAdjective">
                <synset synset="synset" display="display" focus="focus"></synset>
            </div>
        </span>
        <div class="pull-right" ng-show="link">
            <b>Download As:</b>&nbsp;&nbsp;<a target="_self" ng-href="/json/{{link}}">JSON</a>&nbsp;&nbsp;
            <a target="_self" ng-href="/ttl/{{link}}">RDF</a>&nbsp;&nbsp;
            <a target="_self" ng-href="/xml/{{link}}">XML</a>
        </div>
    </div>
</template>
