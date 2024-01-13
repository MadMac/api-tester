<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { requestStore } from './store/requestStore.js'
import { ref, onMounted } from 'vue'
import { RequestResponse, RequestTab } from './models/models'
import { v4 as uuidv4 } from 'uuid';

const apiUrl = ref("");
const tabName = ref("")
const requestType = ref("GET");
const activeTab = ref();

const init_tabs = () => {
  const newTab = {
    uuid: uuidv4(),
    name: "Untitled",
    url: "",
    status: "",
    response: ""
  }
  requestStore.addNewTab(newTab);
}

onMounted(() => {
  if (requestStore.isTabsEmpty()) {
    init_tabs();
  }
  tab_changed()
})

const update_tab_name = () => {
  activeTab.value.name = tabName.value
}

const update_api_url = () => {
  activeTab.value.url = apiUrl.value
}


const tab_changed = () => {
  if (activeTab.value !== undefined) {
    tabName.value = activeTab.value.name
    apiUrl.value = activeTab.value.url
  } else {
    if (requestStore.isTabsEmpty()) {
      init_tabs();
    }
    activeTab.value = requestStore.tabs[requestStore.tabs.length - 1]
  }
}

const add_new_tab = () => {
  const newTab = {
    uuid: uuidv4(),
    name: "Untitled",
    url: "",
    status: "",
    response: ""
  }
  requestStore.addNewTab(newTab);
}

const remove_tab = (remove_tab: RequestTab) => {
  requestStore.removeTab(remove_tab);
  tab_changed();
}

const send_request = () => {
  if (!apiUrl.value) return;
  console.log(requestType)
  requestStore.updateRequestResponse("" as string)
  console.log("type", requestType.value)
  switch (requestType.value) {
    case "GET":
      invoke('send_get_request', { apiUrl: apiUrl.value })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          requestStore.updateRequestResponse(requestResponse.body);
          requestStore.updateStatus(requestResponse.status);
          console.log(requestResponse)
        });
      break;
    case "POST":
      invoke('send_post_request', { apiUrl: apiUrl.value })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          requestStore.updateRequestResponse(requestResponse.body);
          requestStore.updateStatus(requestResponse.status);
          console.log(response)
        });
      break;
    case "PUT":
      invoke('send_put_request', { apiUrl: apiUrl.value })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          requestStore.updateRequestResponse(requestResponse.body);
          requestStore.updateStatus(requestResponse.status);
          console.log(response)
        });
      break;
    case "DELETE":
      invoke('send_delete_request', { apiUrl: apiUrl.value })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          requestStore.updateRequestResponse(requestResponse.body);
          requestStore.updateStatus(requestResponse.status);
          console.log(response)
        });
      break;
    // DELETE
    default:
      console.error("Invalid requestType: " + requestType.value);
  }

}
</script>

<template>
  <div class="container">
    <v-card class="card-container" color="blue-grey" variant="tonal">
      <div class="flex-container flex-row">
        <v-tabs bg-color="blue-grey-darken-4" @click="tab_changed()" v-model="activeTab" class="tab-container">
          <v-tab v-for="n in requestStore.tabs" :value="n">
            {{ n ? n.name.substring(0, 10) : "Error"}}{{ n && n.name.length > 10 ? "..." : ""}} 
            <v-btn icon class="close-tab-button" color="blue-grey-darken-4" height="20" width="20" @click="remove_tab(n)">
              <v-icon size="x-small">mdi-close-circle</v-icon>
            </v-btn>  
          </v-tab>
        </v-tabs>
        <v-btn icon class="new-tab-button" color="blue-grey-darken-1" height="35" width="35" @click="add_new_tab()">
          <v-icon>mdi-plus-circle-outline</v-icon>
        </v-btn>
      </div>
      <div class="flex-container">
        <div class="flex-row">
          <v-text-field label="Name" class="input-col" @input="update_tab_name()" v-model="tabName" hide-details="auto"></v-text-field>
        </div>
        <div class="flex-row">
          <v-select label="Method" :items="['GET', 'POST', 'PUT', 'DELETE']" class="select-col" v-model="requestType"></v-select>
          <v-text-field label="Url" class="input-col" @input="update_api_url()"  v-model="apiUrl"></v-text-field>
          <v-btn block class="button-col" size="x-large" color="blue-grey-lighten-1" @click="send_request()">
            SEND
          </v-btn>
        </div>
        <div class="flex-row-grow">
          <v-table density="compact">
            <thead>
              <tr>
                <th></th>
                <th>Key</th>
                <th>Value</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td width="60"><v-checkbox density="compact" hide-details="true"></v-checkbox></td>
                <td>test</td>
                <td>test</td>
              </tr>
              <tr>
                <td width="60"><v-checkbox density="compact" hide-details="true"></v-checkbox></td>
                <td>test</td>
                <td>test</td>
              </tr>
            </tbody>
          </v-table>
        </div>
        <div class="result-container">
          <v-card class="result-card">
            <v-card-subtitle>
              {{ requestStore.requestStatus != "" ? "Status: " + requestStore.requestStatus : "" }}
            </v-card-subtitle>
            <v-card-text>
              {{ requestStore.requestResponse &&
                JSON.stringify(JSON.parse(requestStore.requestResponse), null, 2) }}
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
}

.flex-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  margin-top: 10px;
}

.flex-row {
  display: flex;
  flex-direction: row;
  margin: 10px;
  flex-grow: 0;
}

.flex-row-grow {
  flex-grow: 1;
  margin: 20px;
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
}

.result-card {
  flex-grow: 1;
  height: 100%;
  margin-left: 10px;
  margin-right: 10px;
  white-space: pre;
  font-family: monospace;
}

.v-card-subtitle {
  text-align: right;
  margin: 10px;
}
</style>
