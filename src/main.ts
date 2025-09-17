import { createApp } from "vue";
import "./index.css";
import App from "./App.vue";
import { useDarkMode } from "@/composables/useDarkMode";

const app = createApp(App);

// Initialize dark mode
if (typeof window !== "undefined") {
  const { initializeDarkMode } = useDarkMode();
  initializeDarkMode();
}

app.mount("#app");
