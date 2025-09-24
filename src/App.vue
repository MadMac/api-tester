<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { requestStore } from "./store/requestStore.js";
import { ref, onMounted, computed } from "vue";
import { RequestResponse, RequestTab, RequestType } from "./models/models";
import { v4 as uuidv4 } from "uuid";

import SideBar from "./components/SideBar.vue";
import TabRow from "./components/TabRow.vue";
import ParameterTable from "./components/ParameterTable.vue";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const additionalFeatures = ref("parameters");
const methodList = [
  RequestType.GET,
  RequestType.POST,
  RequestType.PUT,
  RequestType.DELETE,
];

const default_new_tab = {
  name: "Untitled",
  url: "",
  response: undefined,
  requestType: RequestType.GET,
  parameters: [],
};

const init_tabs = () => {
  console.log("Add tab");
  const newTabData = JSON.parse(JSON.stringify(default_new_tab));

  const newTab = {
    uuid: uuidv4(),
    data: newTabData,
    saved_data: undefined,
  };
  requestStore.addNewTab(newTab);
};

onMounted(() => {
  if (requestStore.isTabsEmpty()) {
    invoke("init_session").then((response) => {
      let requestTabs = response as [];
      if (requestTabs.length === 0) {
        init_tabs();
      } else {
        requestStore.clearTabs();
        requestTabs.forEach((tab) => {
          requestStore.addNewTab(tab);
        });
        requestStore.setActiveTab(requestStore.tabs[0]);
        tab_changed();
      }
    });
  }
  tab_changed();

  document.addEventListener("keydown", (e) => {
    if (e.ctrlKey && e.key === "s") {
      e.preventDefault();
      // Set activetab's saved_data to what is the current data
      requestStore.activeTab.saved_data = JSON.parse(
        JSON.stringify(requestStore.activeTab.data),
      );
      save_session();
    }
  });
});

const tab_changed = () => {
  if (requestStore.activeTab == undefined) {
    if (requestStore.isTabsEmpty()) {
      init_tabs();
    }
    requestStore.setActiveTab(requestStore.tabs[requestStore.tabs.length - 1]);
  }
};

const save_session = () => {
  // Send command to backend to save the session to database
  invoke("save_session", { sessionData: JSON.stringify(requestStore.tabs) });
};

const add_new_tab = () => {
  const newTabData = JSON.parse(JSON.stringify(default_new_tab));

  const newTab = {
    uuid: uuidv4(),
    data: newTabData,
    saved_data: undefined,
  };
  requestStore.addNewTab(newTab);
  save_session();
};

const remove_tab = (remove_tab: RequestTab) => {
  requestStore.removeTab(remove_tab);
  tab_changed();
  save_session();
};

const response_handler = (response: string) => {
  try {
    return JSON.stringify(JSON.parse(response), null, 2);
  } catch (e) {
    return response;
  }
};

const send_request = () => {
  if (!requestStore.activeTab.data.url) return;
  requestStore.activeTab.data.response = {} as RequestResponse;

  // Response data should be empty when sending tab data to the backend
  const tab_data = requestStore.activeTab.data;
  tab_data.response = undefined;
  switch (requestStore.activeTab.data.requestType) {
    case RequestType.GET:
      invoke("send_get_request", { tabData: tab_data }).then((response) => {
        let requestResponse = response as RequestResponse;
        requestStore.activeTab.data.response = requestResponse;
        console.log(requestResponse);
      });
      break;
    case RequestType.POST:
      invoke("send_post_request", { tabData: tab_data }).then((response) => {
        let requestResponse = response as RequestResponse;
        requestStore.activeTab.data.response = requestResponse;
        console.log(response);
      });
      break;
    case RequestType.PUT:
      invoke("send_put_request", { tabData: tab_data }).then((response) => {
        let requestResponse = response as RequestResponse;
        requestStore.activeTab.data.response = requestResponse;
        console.log(response);
      });
      break;
    case RequestType.DELETE:
      invoke("send_delete_request", { tabData: tab_data }).then((response) => {
        let requestResponse = response as RequestResponse;
        requestStore.activeTab.data.response = requestResponse;
        console.log(response);
      });
      break;
    default:
      console.error(
        "Invalid requestType: " + requestStore.activeTab.data.requestType,
      );
  }
};

const status_text_handling = () => {
  if (
    requestStore.activeTab &&
    requestStore.activeTab.data.response &&
    requestStore.activeTab.data.response.status != ""
  ) {
    return "Status: " + requestStore.activeTab.data.response.status;
  }
  return "";
};

const activeTabName = computed({
  get() {
    console.log(requestStore.activeTab);
    if (requestStore.activeTab && requestStore.activeTab.data) {
      return requestStore.activeTab.data.name;
    }
    return "";
  },
  set(newValue: string) {
    requestStore.activeTab.data.name = newValue;
  },
});

const activeTabUrl = computed({
  get() {
    console.log(requestStore.activeTab);
    if (requestStore.activeTab && requestStore.activeTab.data) {
      return requestStore.activeTab.data.url;
    }
    return "";
  },
  set(newValue: string) {
    requestStore.activeTab.data.url = newValue;
  },
});

const activeTabRequestType = computed({
  get() {
    console.log(requestStore.activeTab);
    if (requestStore.activeTab && requestStore.activeTab.data) {
      return requestStore.activeTab.data.requestType;
    }
    return RequestType.GET;
  },
  set(newValue: RequestType) {
    requestStore.activeTab.data.requestType = newValue;
  },
});
</script>

<template>
  <div class="container">
    <SideBar />
    <Card class="card-container p-0">
      <CardContent class="p-0 card-content">
        <div class="flex-container">
          <div class="flex-container flex-row">
            <TabRow
              :tab_changed="tab_changed"
              :remove_tab="remove_tab"
              :add_new_tab="add_new_tab"
            />
          </div>
          <div class="flex-container">
            <div class="flex-row">
              <Input
                placeholder="Name"
                class="input-col"
                v-model="activeTabName"
              />
            </div>
            <div class="flex-row">
              <Select
                :model-value="activeTabRequestType"
                @update:model-value="
                  (value) => (activeTabRequestType = value as RequestType)
                "
              >
                <SelectTrigger class="select-col">
                  <SelectValue placeholder="Method" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem
                    v-for="method in methodList"
                    :key="method"
                    :value="method"
                  >
                    {{ method }}
                  </SelectItem>
                </SelectContent>
              </Select>
              <Input
                placeholder="URL"
                class="input-col"
                v-model="activeTabUrl"
              />
              <Button class="button-col" size="lg" @click="send_request()">
                SEND
              </Button>
            </div>
            <div class="flex-row-grow">
              <Tabs
                :model-value="additionalFeatures"
                @update:model-value="
                  (value) => (additionalFeatures = value as string)
                "
                class="w-full"
              >
                <TabsList>
                  <TabsTrigger value="headers"> Headers </TabsTrigger>
                  <TabsTrigger value="parameters"> Parameters </TabsTrigger>
                  <TabsTrigger value="body"> Body </TabsTrigger>
                </TabsList>
                <TabsContent value="headers"> Headers </TabsContent>
                <TabsContent value="parameters">
                  <ParameterTable />
                </TabsContent>
                <TabsContent value="body"> Body </TabsContent>
              </Tabs>
            </div>
            <div class="result-container">
              <Card class="result-card py-3 gap-0">
                <CardHeader>
                  <p class="text-sm text-muted-foreground text-right">
                    {{ status_text_handling() }}
                  </p>
                </CardHeader>
                <CardContent class="result-box">
                  <pre class="whitespace-pre-wrap font-mono text-sm">{{
                    requestStore.activeTab &&
                    requestStore.activeTab.data.response &&
                    response_handler(requestStore.activeTab.data.response.body)
                  }}</pre>
                </CardContent>
              </Card>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<style scoped>
.tab-container {
  flex-grow: 1;
  margin-right: 20px;
}

.header-bar {
  position: fixed;
  top: 0;
  right: 0;
  z-index: 50;
  padding: 10px;
  display: flex;
  align-items: center;
}

.container {
  width: 100vw;
  height: 100vh;
  padding: 10px;
  display: flex;
  box-sizing: border-box;
  flex-direction: row;
  overflow: hidden;
  max-width: 100vw;
  max-height: 100vh;
}

.card-container {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  margin-left: 10px;
  box-sizing: border-box;
  background-color: hsl(var(--primary) / 0.1);
  min-width: 0;
  max-width: 100vw;
  max-height: 100vh;
  overflow: auto;
}

.flex-container {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  min-height: 0;
  height: 100%;
  justify-content: flex-start;
  padding: 5px;
  overflow: auto;
  max-height: 100%;
}

.flex-row {
  display: flex;
  flex-direction: row;
  margin: 10px;
  flex-shrink: 0;
  max-height: 50px;
  gap: 10px;
  min-width: 0;
  overflow: hidden;
}

.flex-row-grow {
  margin-left: 20px;
  margin-right: 20px;
  padding-top: 20px;
  flex-shrink: 0;
}

.button-col,
.select-col {
  flex-grow: 0;
  flex-basis: 8%;
  min-width: 120px;
  flex-shrink: 0;
}

.input-col {
  flex-grow: 1;
  min-width: 0;
}

.result-container {
  margin: 10px;
  flex-grow: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
  margin-left: 10px;
  margin-right: 10px;
  max-height: 100%;
  overflow: auto;
}

.result-card {
  flex-grow: 1;
  height: 100%;
  box-sizing: border-box;
  margin-left: 10px;
  margin-right: 10px;
  display: flex;
  flex-direction: column;
  max-height: 100%;
}

.result-box {
  box-sizing: border-box;
  overflow-y: auto;
  height: 100%;
  flex-grow: 1;
  max-height: 100%;
}

.card-content {
  flex-grow: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 0;
  overflow: hidden;
}
</style>
