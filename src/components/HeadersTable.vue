<script setup lang="ts">
import { requestStore } from "../store/requestStore.js";
import { RequestHeader } from "../models/models";
import { v4 as uuidv4 } from "uuid";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { X, Plus } from "lucide-vue-next";

const add_header = () => {
  const new_header: RequestHeader = {
    uuid: uuidv4(),
    enabled: true,
    key: "",
    value: "",
  };
  requestStore.activeTab.data.headers.push(new_header);
};

const remove_header = (remove_header: RequestHeader) => {
  requestStore.removeHeader(requestStore.activeTab, remove_header);
};
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
      <TableRow
        v-if="requestStore.activeTab"
        v-for="n in requestStore.activeTab.data.headers"
        :key="n.uuid"
      >
        <TableCell>
          <Checkbox v-model:checked="n.enabled" />
        </TableCell>
        <TableCell>
          <Input
            placeholder="Header"
            class="header-field border-0 h-8 focus-visible:ring-0 focus-visible:ring-offset-0"
            v-model="n.key"
          />
        </TableCell>
        <TableCell>
          <Input
            placeholder="Value"
            class="header-field border-0 h-8 focus-visible:ring-0 focus-visible:ring-offset-0"
            v-model="n.value"
          />
        </TableCell>
        <TableCell>
          <Button
            variant="ghost"
            size="icon"
            class="h-5 w-5"
            @click="remove_header(n)"
          >
            <X class="h-3 w-3" />
          </Button>
        </TableCell>
      </TableRow>
      <TableRow>
        <TableCell colspan="4" class="header-add-button">
          <Button variant="ghost" @click="add_header()">
            <Plus class="h-4 w-4 mr-2" />
            Add Header
          </Button>
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>

<style scoped>
.header-add-button {
  width: 100%;
  text-align: center;
  padding-top: 5px !important;
  padding-bottom: 5px !important;
}

.checkbox-column {
  width: 3%;
}

.header-field {
  background: transparent;
}
</style>
