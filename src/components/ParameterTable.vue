<script setup lang="ts">
import { requestStore } from '../store/requestStore.js'
import { RequestParameter } from '../models/models'
import { v4 as uuidv4 } from 'uuid';

const add_parameter = () => {
  const new_parameter: RequestParameter = {
    uuid: uuidv4(),
    enabled: true,
    key: "",
    value: ""
  }
  requestStore.activeTab.data.parameters.push(new_parameter);
}

const remove_parameter = (remove_parameter: RequestParameter) => {
  requestStore.removeParameter(requestStore.activeTab, remove_parameter);
}

</script>

<template>
  <v-table density="compact">
    <thead>
      <tr>
        <th class="checkbox-column"></th>
        <th>Key</th>
        <th>Value</th>
        <th class="checkbox-column"></th>
      </tr>
    </thead>
    <tbody>
      <tr v-if="requestStore.activeTab" v-for="n in requestStore.activeTab.data.parameters" :value="n" :key="n.uuid">
        <td><v-checkbox density="compact" hide-details="auto" v-model="n.enabled"></v-checkbox></td>
        <td>
          <v-text-field placeholder="Parameter" variant="plain" hide-details="auto" density="compact"
            class="parameter-field" v-model="n.key"></v-text-field>
        </td>
        <td>
          <v-text-field placeholder="Value" variant="plain" hide-details="auto" v-model="n.value" density="compact"
            class="parameter-field"></v-text-field>
        </td>
        <td>
          <v-btn icon height="20" width="20" @click="remove_parameter(n)">
            <v-icon size="x-small">mdi-close-circle</v-icon>
          </v-btn>
        </td>
      </tr>
      <tr>
        <td colspan="4" class="parameter-add-button">
          <v-btn variant="flat" @click="add_parameter()">
            <v-icon>mdi-plus-circle-outline</v-icon>
          </v-btn>
        </td>
      </tr>
    </tbody>
  </v-table>
</template>

<style scoped>
.parameter-add-button {
  width: 100%;
  text-align: center;
  padding-top: 5px !important;
  padding-bottom: 5px !important;
}

.checkbox-column {
  width: 3%;
}
</style>