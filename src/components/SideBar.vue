<script setup lang="ts">
</script>

<template>
	<v-card class="side-container" color="light-blue" variant="tonal">
      <div class="flex-container flex-row">Test</div>
    </v-card>
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
}
</style>