<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { requestStore } from './store/requestStore.js'
import { ref, onMounted } from 'vue'
import { RequestResponse, RequestTab, RequestParameter } from './models/models'
import { v4 as uuidv4 } from 'uuid';

const apiUrl = ref("");
const tabName = ref("")
const requestType = ref("GET");
const activeTab = ref();

const init_tabs = () => {
  const newTabData = {
    name: "Untitled",
    url: "",
    response: undefined,
    parameters: []
  }

  const newTab = {
    uuid: uuidv4(),
    data: newTabData,
    saved_data: undefined
  }
  requestStore.addNewTab(newTab);
}

onMounted(() => {
  if (requestStore.isTabsEmpty()) {
    invoke('init_session').then((response) => {
      console.log(response);
    });
    init_tabs();
    console.log(requestStore.tabs)
  }
  tab_changed()
})

const update_tab_name = () => {
  activeTab.value.data.name = tabName.value
}

const update_api_url = () => {
  activeTab.value.data.url = apiUrl.value
}


const tab_changed = () => {
  if (activeTab.value !== undefined) {
    tabName.value = activeTab.value.data.name
    apiUrl.value = activeTab.value.data.url
  } else {
    if (requestStore.isTabsEmpty()) {
      init_tabs();
    }
    activeTab.value = requestStore.tabs[requestStore.tabs.length - 1]
  }
}

const save_session = () => {
  // Send command to backend to save the session to database
  invoke('save_session', { sessionData: JSON.stringify(requestStore.tabs) });
}

const add_new_tab = () => {
  const newTabData = {
    name: "Untitled",
    url: "",
    response: undefined,
    parameters: []
  }

  const newTab = {
    uuid: uuidv4(),
    data: newTabData,
    saved_data: undefined
  }
  requestStore.addNewTab(newTab);
  save_session();
}

const remove_tab = (remove_tab: RequestTab) => {
  requestStore.removeTab(remove_tab);
  tab_changed();
  save_session();
}

const remove_parameter = (remove_parameter: RequestParameter) => {
  requestStore.removeParameter(activeTab.value, remove_parameter);
}

const add_parameter = () => {
  const new_parameter: RequestParameter = {
    uuid: uuidv4(),
    enabled: true,
    key: "",
    value: ""
  }
  activeTab.value.data.parameters.push(new_parameter);
}

const send_request = () => {
  if (!apiUrl.value) return;
  activeTab.value.data.response = "";
  switch (requestType.value) {
    case "GET":
      invoke('send_get_request', { apiUrl: apiUrl.value })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          activeTab.value.data.response = requestResponse
          console.log(requestResponse)
        });
      break;
    case "POST":
      invoke('send_post_request', { apiUrl: apiUrl.value })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          activeTab.value.data.response = requestResponse
          console.log(response)
        });
      break;
    case "PUT":
      invoke('send_put_request', { apiUrl: apiUrl.value })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          activeTab.value.data.response = requestResponse
          console.log(response)
        });
      break;
    case "DELETE":
      invoke('send_delete_request', { apiUrl: apiUrl.value })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          activeTab.value.data.response = requestResponse
          console.log(response)
        });
      break;
    default:
      console.error("Invalid requestType: " + requestType.value);
  }

}
</script>

<template>
  <div class="container">
    <v-card class="side-container" color="light-blue" variant="tonal">
      <div class="flex-container flex-row">Test</div>
    </v-card>
    <v-card class="card-container" color="light-blue" variant="tonal">
      <div class="flex-container flex-row">
        <v-tabs bg-color="light-blue-darken-4" @click="tab_changed()" v-model="activeTab" class="tab-container"
          show-arrows>
          <v-tab v-for="n in requestStore.tabs" :value="n">
            {{ n ? n.data.name.substring(0, 10) : "Error" }}{{ n && n.data.name.length > 10 ? "..." : "" }}
            <v-btn icon class="close-tab-button" color="light-blue-darken-4" height="20" width="20"
              @click="remove_tab(n)">
              <v-icon size="x-small">mdi-close-circle</v-icon>
            </v-btn>
          </v-tab>
        </v-tabs>
        <v-btn icon class="new-tab-button" color="light-blue-darken-1" height="35" width="35" @click="add_new_tab()">
          <v-icon>mdi-plus-circle-outline</v-icon>
        </v-btn>
      </div>
      <div class="flex-container">
        <div class="flex-row">
          <v-text-field label="Name" class="input-col" @input="update_tab_name()" v-model="tabName"
            hide-details="auto"></v-text-field>
        </div>
        <div class="flex-row">
          <v-select label="Method" :items="['GET', 'POST', 'PUT', 'DELETE']" class="select-col"
            v-model="requestType"></v-select>
          <v-text-field label="Url" class="input-col" @input="update_api_url()" v-model="apiUrl"></v-text-field>
          <v-btn block class="button-col" size="x-large" color="light-blue-darken-1" @click="send_request()">
            SEND
          </v-btn>
        </div>
        <div class="flex-row-grow">
          <v-table density="compact">
            <thead>
              <tr>
                <th class="checkbox-column"></th>
                <th>Key</th>
                <th>Value</th>
                <th class="checkbox-column"></th>
              </tr>
            </thead>
            <tbody>
              <tr v-if="activeTab" v-for="n in activeTab.data.parameters" :value="n" :key="n.uuid">
                <td><v-checkbox density="compact" hide-details="auto" v-model="n.enabled"></v-checkbox></td>
                <td>
                  <v-text-field placeholder="Parameter" variant="plain" hide-details="auto" density="compact"
                    class="parameter-field" v-model="n.key"></v-text-field>
                </td>
                <td>
                  <v-text-field placeholder="Value" variant="plain" hide-details="auto" v-model="n.value"
                    density="compact" class="parameter-field"></v-text-field>
                </td>
                <td>
                  <v-btn icon height="20" width="20" @click="remove_parameter(n)">
                    <v-icon size="x-small">mdi-close-circle</v-icon>
                  </v-btn>
                </td>
              </tr>
              <tr>
                <td colspan="4" class="parameter-add-button">
                  <v-btn variant="flat" @click="add_parameter()">
                    <v-icon>mdi-plus-circle-outline</v-icon>
                  </v-btn>
                </td>
              </tr>
            </tbody>
          </v-table>
        </div>
        <div class="result-container">
          <v-card class="result-card">
            <v-card-subtitle>
              {{ activeTab && activeTab.data.response && activeTab.data.response.status != "" ? "Status: " +
          activeTab.data.response.status : "" }}
            </v-card-subtitle>
            <v-card-text class="result-box" scrollable>
              {{ activeTab && activeTab.data.response &&
          JSON.stringify(JSON.parse(activeTab.data.response.body), null, 2) }}
            </v-card-text>
          </v-card>
        </div>
      </div>
    </v-card>
  </div>
</template>

<style scoped>
.tab-container {
  flex-grow: 1;
  margin-right: 20px;
}

.new-tab-button {
  margin-top: auto;
  margin-bottom: auto;
  margin-left: auto;
  margin-right: 10px;
  max-width: 35px;
}

.close-tab-button {
  margin-top: auto;
  margin-bottom: auto;
  margin-left: 10px;
  margin-right: 0px;
  opacity: 0;
}

.close-tab-button:hover {
  opacity: 1;
}

.container {
  width: 100%;
  height: 100%;
  padding: 10px;
  display: flex;
  box-sizing: border-box;
  flex-direction: row;
}

.card-container {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  margin-left: 10px;
  box-sizing: border-box;
  justify-content: space-evenly;
}

.side-container {
  min-width: 100px;
  resize: horizontal;
  width: 300px;
  max-width: 600px;
  ;
}

.flex-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  margin-top: 10px;
  justify-content: space-evenly;
}

.flex-row {
  display: flex;
  flex-direction: row;
  margin: 10px;
  flex-grow: 1;
  max-height: 50px;
}

.flex-row-grow {
  margin-left: 20px;
  margin-right: 20px;
  padding-top: 20px;
}

.button-col,
.select-col {
  flex-grow: 0;
  flex-basis: 8%;
  margin-left: 10px;
  margin-right: 10px;
  min-width: 120px;
}

.input-col {
  flex-grow: 1;
  margin-left: 10px;
  margin-right: 10px;
}

.result-container {
  margin: 10px;
  flex-grow: 1;
  box-sizing: border-box;
  margin-left: 10px;
  margin-right: 10px;
}

.result-card {
  flex-grow: 1;
  height: 100%;
  box-sizing: border-box;
  margin-left: 10px;
  margin-right: 10px;
  white-space: pre;
  font-family: monospace;
}

.result-box {
  box-sizing: border-box;
  overflow-y: scroll;
  height: 100%;
}

.v-card-subtitle {
  text-align: right;
  margin: 10px;
}

.parameter-add-button {
  width: 100%;
  text-align: center;
  padding-top: 5px !important;
  padding-bottom: 5px !important;
}

.checkbox-column {
  width: 3%;
}
</style>
