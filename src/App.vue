<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { requestStore } from './store/requestStore.js'
import { ref, onMounted, computed } from 'vue'
import { RequestResponse, RequestTab } from './models/models'
import { v4 as uuidv4 } from 'uuid';

import SideBar from './components/SideBar.vue';
import TabRow from './components/TabRow.vue';
import ParameterTable from './components/ParameterTable.vue';

const requestType = ref("GET");
const additionalFeatures = ref();

const init_tabs = () => {
  console.log("Add tab")
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
      let requestTabs = response as []
      if (requestTabs.length === 0) {
        init_tabs();
      } else {
        requestStore.clearTabs();
        requestTabs.forEach((tab) => {
          requestStore.addNewTab(tab);
        })
        requestStore.setActiveTab(requestStore.tabs[0])
        tab_changed()
      }
    });
  }
  tab_changed()

  document.addEventListener('keydown', e => {
    if (e.ctrlKey && e.key === 's') {
      e.preventDefault();
      // Set activetab's saved_data to what is the current data
      requestStore.activeTab.saved_data = JSON.parse(JSON.stringify(requestStore.activeTab.data))
      save_session();
    }
  });
})

const tab_changed = () => {
  if (requestStore.activeTab == undefined) {
    if (requestStore.isTabsEmpty()) {
      init_tabs();
    }
    requestStore.setActiveTab(requestStore.tabs[requestStore.tabs.length - 1])
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

const response_handler = (response: string) => {
  try {
    return JSON.stringify(JSON.parse(response), null, 2);
  } catch (e) {
    return response;
  }
}

const send_request = () => {
  if (!requestStore.activeTab.data.url) return;
  requestStore.activeTab.data.response = {} as RequestResponse;

  // Response data should be empty when sending tab data to the backend
  const tab_data = requestStore.activeTab.data
  tab_data.response = undefined;

  switch (requestType.value) {
    case "GET":
      invoke('send_get_request', { tabData: tab_data })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          requestStore.activeTab.data.response = requestResponse
          console.log(requestResponse)
        });
      break;
    case "POST":
      invoke('send_post_request', { tabData: tab_data })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          requestStore.activeTab.data.response = requestResponse
          console.log(response)
        });
      break;
    case "PUT":
      invoke('send_put_request', { tabData: tab_data })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          requestStore.activeTab.data.response = requestResponse
          console.log(response)
        });
      break;
    case "DELETE":
      invoke('send_delete_request', { tabData: tab_data })
        .then((response) => {
          let requestResponse = response as RequestResponse;
          requestStore.activeTab.data.response = requestResponse
          console.log(response)
        });
      break;
    default:
      console.error("Invalid requestType: " + requestType.value);
  }

}

const status_text_handling = () => {
  if (requestStore.activeTab && requestStore.activeTab.data.response && requestStore.activeTab.data.response.status != "") {
    return "Status: " + requestStore.activeTab.data.response.status
  }
  return ""
}

const activeTabName = computed({
  get() {
    console.log(requestStore.activeTab)
    if (requestStore.activeTab && requestStore.activeTab.data) {
      return requestStore.activeTab.data.name;
    }
    return ""
  },
  set(newValue: string) {
    requestStore.activeTab.data.name = newValue;
  }
})

const activeTabUrl = computed({
  get() {
    console.log(requestStore.activeTab)
    if (requestStore.activeTab && requestStore.activeTab.data) {
      return requestStore.activeTab.data.url;
    }
    return ""
  },
  set(newValue: string) {
    requestStore.activeTab.data.url = newValue;
  }
})

</script>

<template>
  <div class="container">
    <SideBar />
    <v-card class="card-container" color="light-blue" variant="tonal">
      <div class="flex-container flex-row">
        <TabRow :tab_changed="tab_changed" :remove_tab="remove_tab" :add_new_tab="add_new_tab" />
      </div>
      <div class="flex-container">
        <div class="flex-row">
          <v-text-field label="Name" class="input-col" v-model="activeTabName" hide-details="auto"></v-text-field>
        </div>
        <div class="flex-row">
          <v-select label="Method" :items="['GET', 'POST', 'PUT', 'DELETE']" class="select-col"
            v-model="requestType"></v-select>
          <v-text-field label="Url" class="input-col" v-model="activeTabUrl"></v-text-field>
          <v-btn block class="button-col" size="x-large" color="light-blue-darken-1" @click="send_request()">
            SEND
          </v-btn>
        </div>
        <div class="flex-row-grow">
          <v-tabs align-tabs="center" color="white" height="60" slider-color="#2196F3" v-model="additionalFeatures"
            class="tab-bottom-margin">
            <v-tab value="headers">
              Headers
            </v-tab>
            <v-tab value="parameters">
              Parameters
            </v-tab>
            <v-tab value="body">
              Body
            </v-tab>
          </v-tabs>
          <v-tabs-window v-model="additionalFeatures">
            <v-tabs-window-item value="headers">
              Headers
            </v-tabs-window-item>
            <v-tabs-window-item value="parameters">
              <ParameterTable />
            </v-tabs-window-item>
            <v-tabs-window-item value="body">
              Body
            </v-tabs-window-item>
          </v-tabs-window>
        </div>
        <div class="result-container">
          <v-card class="result-card">
            <v-card-subtitle>
              {{ status_text_handling() }}
            </v-card-subtitle>
            <v-card-text class="result-box" scrollable>
              {{ requestStore.activeTab && requestStore.activeTab.data.response &&
          response_handler(requestStore.activeTab.data.response.body) }}
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

.tab-bottom-margin {
  margin-bottom: 20px;
}
</style>
