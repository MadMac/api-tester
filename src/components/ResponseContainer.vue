<script setup lang="ts">
import { requestStore } from "../store/requestStore.js";
import { Card, CardContent, CardHeader } from "@/components/ui/card";

const response_handler = (response: string) => {
  try {
    return JSON.stringify(JSON.parse(response), null, 2);
  } catch (e) {
    return response;
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
</script>

<template>
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
</template>

<style scoped>
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
</style>
