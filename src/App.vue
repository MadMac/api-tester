<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { requestStore } from './store/requestStore.js'
import { ref } from 'vue'
import { RequestResponse } from './models/models'
import { v4 as uuidv4 } from 'uuid';

const apiUrl = ref("");
const tabName = ref("")
const requestType = ref("GET");
const activeTab = ref();

if (requestStore.tabs.length === 0) {
  const newTab = {
    uuid: uuidv4(),
    name: "",
    url: "",
    status: "",
    response: ""
  }
  requestStore.addNewTab(newTab);
}

const add_new_tab = () => {
  const newTab = {
    uuid: uuidv4(),
    name: "",
    url: "",
    status: "",
    response: ""
  }
  requestStore.addNewTab(newTab);
}

const send_get_request = () => {
  if (!apiUrl.value) return;
  console.log(requestType)
  requestStore.updateRequestResponse("" as string)
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
          requestStore.updateRequestResponse(response as string);
          console.log(response)
        });
      break;
    case "PUT":
      invoke('send_put_request', { apiUrl: apiUrl.value })
        .then((response) => {
          requestStore.updateRequestResponse(response as string);
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
        <v-tabs bg-color="blue-grey-darken-4" v-model="activeTab" class="tab-container">
          <v-tab v-for="n in requestStore.tabs" :value="n">
            Item
          </v-tab>

        </v-tabs>
        <v-btn icon class="new-tab-button" color="blue-grey-darken-1" height="35" width="35" @click="add_new_tab()">
          <v-icon>mdi-plus-circle-outline</v-icon>
        </v-btn>
      </div>
      <div class="flex-container">
        <div class="flex-row">
          <v-text-field label="Name" class="input-col" v-model="tabName" hide-details="auto"></v-text-field>
        </div>
        <div class="flex-row">
          <v-select label="Select" :items="['GET', 'POST', 'PUT']" class="select-col" v-model="requestType"></v-select>
          <v-text-field label="Input" class="input-col" v-model="apiUrl"></v-text-field>
          <v-btn block class="button-col" size="x-large" color="blue-grey-lighten-1" @click="send_get_request()">
            SEND
          </v-btn>
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

.button-col,
.select-col {
  flex-grow: 0;
  flex-basis: 8%;
  margin-left: 10px;
  margin-right: 10px;
  min-width: 110px;
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
