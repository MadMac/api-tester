<script setup lang="ts">
import { Card, CardContent } from "@/components/ui/card";
import DarkModeToggle from "./DarkModeToggle.vue";
</script>

<template>
  <Card class="side-container">
    <div class="header-bar">
      <div class="flex items-center gap-2">
        <DarkModeToggle />
      </div>
    </div>
    <CardContent class="flex-container flex-row p-4"> Test </CardContent>
  </Card>
</template>

<style scoped>
.header-bar {
  margin-left: 10px;
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

.side-container {
  min-width: 100px;
  resize: horizontal;
  width: 300px;
  max-width: 600px;
  background-color: hsl(var(--primary) / 0.1);
}
</style>
