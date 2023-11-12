import { reactive } from 'vue'

export const requestStore = reactive({
  requestResponse: "",
  requestStatus: "",
  updateRequestResponse(requestResponse: string) {
    this.requestResponse = requestResponse;
  },
  updateStatus(requestStatus: string) {
    this.requestStatus = requestStatus;
  }
})