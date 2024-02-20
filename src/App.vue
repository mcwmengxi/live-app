<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import { appWindow } from '@tauri-apps/api/window'
import { onMounted, ref } from 'vue'
import Live2d from './components/Live2d.vue'
import MMD from './components/mmd/MMD.vue'

const isMMD = ref<boolean>(false)

onMounted(async () => {
  // await appWindow.setIgnoreCursorEvents(true)
  // await appWindow.setDecorations(true)
})
</script>

<template>
  <div class="">
    <Suspense>
      <Live2d v-if="isMMD" />
      <MMD v-else />
      <template #fallback>
        <p>Loading...</p>
      </template>
    </Suspense>
  </div>
</template>

<style scoped></style>
