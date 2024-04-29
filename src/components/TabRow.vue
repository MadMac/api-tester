<script setup lang="ts">
import { requestStore } from '../store/requestStore.js'
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

</script>

<template>
	<v-tabs bg-color="light-blue-darken-4" @click="props.tab_changed()" v-model="requestStore.activeTab"
		class="tab-container" show-arrows>
		<v-tab v-for="n in requestStore.tabs" :value="n">
			{{ n ? n.data.name.substring(0, 10) : "Error" }}{{ n && n.data.name.length > 10 ? "..." : "" }}
			<v-btn icon class="close-tab-button" color="light-blue-darken-4" height="20" width="20"
				@click="props.remove_tab(n)">
				<v-icon size="x-small">mdi-close-circle</v-icon>
			</v-btn>
		</v-tab>
	</v-tabs>
	<v-btn icon class="new-tab-button" color="light-blue-darken-1" height="35" width="35" @click="add_new_tab()">
		<v-icon>mdi-plus-circle-outline</v-icon>
	</v-btn>
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

.close-tab-button {
	margin-top: auto;
	margin-bottom: auto;
	margin-left: 10px;
	margin-right: 0px;
	opacity: 0;
}

.close-tab-button:hover {
	opacity: 1;
}
</style>