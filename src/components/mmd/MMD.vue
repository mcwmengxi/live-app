<template>
  <div class="w-full h-full p-1">
    <canvas ref="mmd_canvas" class="w-full h-full"></canvas>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { Engine } from '@babylonjs/core'
import { BaseRuntime, MMDOptions } from './MMDRuntime'
import { Store } from 'tauri-plugin-store-api'
import { join, resourceDir } from '@tauri-apps/api/path'
import { SceneBuilder } from './MMDBuilder'

const mmd_canvas = ref()
const resourceDirPath = await resourceDir()
const path = await join(resourceDirPath, 'data', 'settings_mmd.json')
const store = new Store(path)
const mmdUrl = (await store.get('mmd_url')) as string
const sVolume = ref((await store.get('volume')) as number)
const sMmdCamera = ref((await store.get('is_mmd_camera')) as boolean)
const sPaused = ref((await store.get('is_paused')) as boolean)

function reload() {
  location.reload()
}
async function ListenEvent() {
  await listen('event_model_url', (_event: unknown) => {
    console.log('event_model_url')
    reload()
  })
  // await listen('event_open_settings', (_event: unknown) => {
  //   openSettings()
  // })
}

onMounted(() => {
  console.log('MMD onMounted')
  ListenEvent()
  const mmdCanvas = mmd_canvas.value as HTMLCanvasElement
  mmdCanvas.addEventListener(
    'pointerdown',
    (event) => {
      event.preventDefault()
      event.stopPropagation()
    },
    true
  )
  // 创建引擎，第二个参数为开启抗锯齿
  const engine = new Engine(mmdCanvas, true)

  const options: MMDOptions = {
    mmdUrl: mmdUrl,
    mmdCamera: sMmdCamera.value,
    paused: sPaused.value,
    volume: sVolume.value
  }
  BaseRuntime.Create({
    engine,
    canvas: mmdCanvas,
    sceneBuilder: new SceneBuilder(),
    mmdOptions: options
  }).then((runtime) => runtime.run())
})
</script>
