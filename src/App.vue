<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { ref } from 'vue'

const apiUrl = ref("");
const requestType = ref("");

const send_get_request = () => {
  console.log(requestType)
  invoke('send_get_request', { apiUrl: apiUrl.value })
    .then((response) => {
      console.log(response)
    });
}

</script>

<template>
  <div class="container mx-auto">
    <select class="px-3 py-2.5 mx-10 w-32 bg-neutral-900 border rounded-[7px] border-gray-500 bg-gray-700 text-white"
      v-model="requestType">
      <option selected>GET</option>
      <option>POST</option>
      <option>PUT</option>
    </select>
    <input class="px-3 py-2.5 w-96 bg-neutral-900 border rounded-[7px] border-gray-500" v-model="apiUrl" />
    <button class="btn btn-blue mx-10" @click="send_get_request()">
      SEND
    </button>
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
