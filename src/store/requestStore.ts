import { reactive } from 'vue'
import { RequestTab } from '../models/models'

export const requestStore = reactive({
  requestResponse: "",
  requestStatus: "",
  updateRequestResponse(requestResponse: string) {
    this.requestResponse = requestResponse;
  },
  updateStatus(requestStatus: string) {
    this.requestStatus = requestStatus;
  },
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
  isTabsEmpty() {
    return this.tabs.length === 0
  }
})