import { ref, onMounted, watch, readonly } from "vue";

const isDark = ref(false);

export function useDarkMode() {
  const toggleDarkMode = () => {
    isDark.value = !isDark.value;
  };

  const setDarkMode = (dark: boolean) => {
    isDark.value = dark;
  };

  const initializeDarkMode = () => {
    // Check for saved theme preference or default to system preference
    const savedTheme = localStorage.getItem("theme");
    const prefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)",
    ).matches;

    if (savedTheme === "dark" || (savedTheme === null && prefersDark)) {
      isDark.value = true;
    } else if (savedTheme === "light") {
      isDark.value = false;
    } else {
      isDark.value = prefersDark;
    }

    // Apply the theme to the document
    updateDocumentClass();
  };

  const updateDocumentClass = () => {
    if (isDark.value) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  };

  // Watch for changes and update localStorage and document class
  watch(isDark, (newValue) => {
    localStorage.setItem("theme", newValue ? "dark" : "light");
    updateDocumentClass();
  });

  return {
    isDark: readonly(isDark),
    toggleDarkMode,
    setDarkMode,
    initializeDarkMode,
  };
}

// Initialize dark mode when the module is imported
if (typeof window !== "undefined") {
  const { initializeDarkMode } = useDarkMode();
  initializeDarkMode();
}
