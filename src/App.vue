<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { requestStore } from './store/requestStore.js'
import { ref } from 'vue'
import { RequestResponse } from './models/models'

const apiUrl = ref("");
const requestType = ref("GET");

const send_get_request = () => {
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
  <div class="flex-container">
    <div class="flex-row">
      <v-select label="Select" :items="['GET', 'POST', 'PUT']" class="select-col" v-model="requestType"></v-select>
      <v-text-field label="Input" class="input-col" v-model="apiUrl"></v-text-field>
      <v-btn block class="button-col" size="x-large" color="#9ed2ae" @click="send_get_request()">
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
</template>

<style scoped>
.flex-container {
  display: flex;
  flex-direction: column;
  height: 100%;
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
