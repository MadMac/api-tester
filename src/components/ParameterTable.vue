<script setup lang="ts">
import { requestStore } from '../store/requestStore.js'
import { RequestParameter } from '../models/models'
import { v4 as uuidv4 } from 'uuid';
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Checkbox } from '@/components/ui/checkbox'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { X, Plus } from 'lucide-vue-next'

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
  <Table>
    <TableHeader>
      <TableRow>
        <TableHead class="checkbox-column"></TableHead>
        <TableHead>Key</TableHead>
        <TableHead>Value</TableHead>
        <TableHead class="checkbox-column"></TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow v-if="requestStore.activeTab" v-for="n in requestStore.activeTab.data.parameters" :key="n.uuid">
        <TableCell>
          <Checkbox v-model:checked="n.enabled" />
        </TableCell>
        <TableCell>
          <Input
            placeholder="Parameter"
            class="parameter-field border-0 h-8 focus-visible:ring-0 focus-visible:ring-offset-0"
            v-model="n.key"
          />
        </TableCell>
        <TableCell>
          <Input
            placeholder="Value"
            class="parameter-field border-0 h-8 focus-visible:ring-0 focus-visible:ring-offset-0"
            v-model="n.value"
          />
        </TableCell>
        <TableCell>
          <Button
            variant="ghost"
            size="icon"
            class="h-5 w-5"
            @click="remove_parameter(n)"
          >
            <X class="h-3 w-3" />
          </Button>
        </TableCell>
      </TableRow>
      <TableRow>
        <TableCell colspan="4" class="parameter-add-button">
          <Button variant="ghost" @click="add_parameter()">
            <Plus class="h-4 w-4 mr-2" />
            Add Parameter
          </Button>
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
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

.parameter-field {
  background: transparent;
}
</style>
