<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { requestStore } from './store/requestStore.js'
import { ref } from 'vue'

const apiUrl = ref("");
const requestType = ref("GET");

const send_get_request = () => {
  console.log(requestType)
  invoke('send_get_request', { apiUrl: apiUrl.value })
    .then((response) => {
      requestStore.updateRequestResponse(response as string);
      console.log(response)
    });
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
      <v-card class="result-card" subtitle="Subtitle">
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
  min-width: 100px;
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
}
</style>
