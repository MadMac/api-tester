<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Moon, Sun } from "lucide-vue-next";
import { useDarkMode } from "@/composables/useDarkMode";

const { isDark, toggleDarkMode } = useDarkMode();
</script>

<template>
  <Button
    variant="ghost"
    size="icon"
    @click="toggleDarkMode"
    class="h-9 w-9"
    :aria-label="isDark ? 'Switch to light mode' : 'Switch to dark mode'"
  >
    <Sun v-if="isDark" class="h-4 w-4 transition-all" />
    <Moon v-else class="h-4 w-4 transition-all" />
  </Button>
</template>
