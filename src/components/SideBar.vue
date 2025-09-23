<script setup lang="ts">
import { Card, CardContent } from "@/components/ui/card";
</script>

<template>
  <Card class="side-container">
    <CardContent class="flex-container flex-row p-4"> Test </CardContent>
  </Card>
</template>

<style scoped>
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
