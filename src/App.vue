<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import { appWindow } from '@tauri-apps/api/window'
import { onMounted } from 'vue'
import Live2d from './components/Live2d.vue'

onMounted(async () => {
  // await appWindow.setIgnoreCursorEvents(true)
  // await appWindow.setDecorations(true)
})
</script>

<template>
  <div class="">
    <Suspense>
      <Live2d />
      <template #fallback>
        <p>Loading...</p>
      </template>
    </Suspense>
  </div>
</template>

<style scoped></style>
