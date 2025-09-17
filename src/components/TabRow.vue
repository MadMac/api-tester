<script setup lang="ts">
import { requestStore } from '../store/requestStore.js'
import { Button } from '@/components/ui/button'
import { Plus, X, AlertCircle } from 'lucide-vue-next'


const props = defineProps({
  tab_changed: {
    type: Function,
    required: true
  },
  remove_tab: {
    type: Function,
    required: true
  },
  add_new_tab: {
    type: Function,
    required: true
  }
})

const handleTabClick = (tab: any) => {
  requestStore.setActiveTab(tab)
  props.tab_changed()
}

const isActiveTab = (tab: any) => {
  return requestStore.activeTab?.uuid === tab.uuid
}

</script>

<template>
  <div class="flex items-center w-full">
    <div class="flex bg-muted p-1 rounded-md tab-container">
      <button
        v-for="n in requestStore.tabs"
        :key="n.uuid"
        @click="handleTabClick(n)"
        :class="[
          'relative group h-8 px-3 rounded-sm transition-colors flex items-center gap-1',
          isActiveTab(n)
            ? 'bg-background text-foreground shadow-sm'
            : 'text-muted-foreground hover:text-foreground hover:bg-background/50'
        ]"
      >
        <span class="truncate max-w-20 text-sm">
          {{ n ? n.data.name.substring(0, 10) : "Error" }}{{ n && n.data.name.length > 10 ? "..." : "" }}
        </span>
        <AlertCircle v-if="!requestStore.isTabSaved(n)" class="h-3 w-3 text-orange-500" />
        <Button
          variant="ghost"
          size="icon"
          class="h-4 w-4 opacity-0 group-hover:opacity-100 hover:bg-destructive/20"
          @click.stop="props.remove_tab(n)"
        >
          <X class="h-3 w-3" />
        </Button>
      </button>
    </div>
    <Button
      variant="outline"
      size="icon"
      class="new-tab-button h-8 w-8 ml-2"
      @click="add_new_tab()"
    >
      <Plus class="h-4 w-4" />
    </Button>
  </div>
</template>

<style scoped>
.tab-container {
  flex-grow: 1;
  margin-right: 20px;
}

.new-tab-button {
  margin-top: auto;
  margin-bottom: auto;
  margin-left: auto;
  margin-right: 10px;
  max-width: 35px;
}
</style>
