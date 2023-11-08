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
  <div class="mx-10 py-10 flex flex-col h-full">
    <div class="flex flex-row">
      <select class="px-3 py-2.5 mx-10 basis-1/12 bg-neutral-900 border rounded-[7px] border-gray-500  text-white"
        v-model="requestType">
        <option selected>GET</option>
        <option>POST</option>
        <option>PUT</option>
      </select>
      <input class="px-3 py-2.5 grow bg-neutral-900 border rounded-[7px] border-gray-500" v-model="apiUrl" />
      <button class="btn btn-blue mx-10 basis-1/12" @click="send_get_request()">
        SEND
      </button>
    </div>
    <div class="mt-10 grow mx-10 mt-10">
      <div
        class="h-full w-full grow border font-mono text-sm p-4 border-gray-500 bg-neutral-900 text-white whitespace-pre">
        {{ requestStore.requestResponse && JSON.stringify(JSON.parse(requestStore.requestResponse), null, 2) }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.btn {
  @apply font-bold py-2 px-4 rounded;
}

.btn-blue {
  @apply bg-blue-500 text-white;
}

.btn-blue:hover {
  @apply bg-blue-700;
}
</style>
