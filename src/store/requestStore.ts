import { reactive } from 'vue'

export const requestStore = reactive({
  requestResponse: "",
  updateRequestResponse(requestResponse: string) {
    this.requestResponse = requestResponse;
  }
})