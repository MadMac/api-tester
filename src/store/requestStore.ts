import { reactive } from 'vue'
import { RequestTab, RequestParameter } from '../models/models'

export const requestStore = reactive({
  activeTab: undefined as unknown as RequestTab,
  tabs: [] as RequestTab[],
  addNewTab(tab: RequestTab) {
    this.tabs.push(tab);
  },
  removeTab(tab: RequestTab) {  
    const index = this.tabs.indexOf(tab);
    if (index !== -1) {
      this.tabs.splice(index, 1);
    }
  },
  removeParameter(tab: RequestTab, parameter: RequestParameter) {
    const index = this.tabs.indexOf(tab);
    if (index !== -1) {
      const parIndex = this.tabs[index].data.parameters.indexOf(parameter);
      if (parIndex !== -1) {
        this.tabs[index].data.parameters.splice(parIndex, 1);
      }
    }
  },
  isTabsEmpty() {
    return this.tabs.length === 0
  },
  clearTabs() {
    this.tabs = [];
  }
})