<template>
  <div
    data-tauri-drag-region
    @click="isHovered = !isHovered"
    class="w-full h-full border border-transparent border-dashed static visible"
  >
    <canvas
      data-tauri-drag-region
      ref="live2d_canvas"
      class="w-full h-full"
    ></canvas>
    <div
      class="absolute inset-y-0 flex flex-col right-0 mx-4 my-4 py-4 px-2 space-y-4 backdrop-blur-3xl hover:bg-white/30"
    >
      <ul v-if="isModelReady">
        <li
          class="w-8 h-6 color-transparent hover:color-gray"
          :class="isHovered ? 'color-gray' : ''"
          @click="reload"
        >
          <ArrowPathIcon class="w-6 h-6" />
        </li>
        <li
          class="w-8 h-8 color-transparent hover:color-gray"
          :class="isHovered ? 'color-gray' : ''"
          
        >
          <SpeakerWaveIcon v-if="sModelVoice" class="w-6 h-6"  @click="changeModelVoice(!sModelVoice)"/>
          <SpeakerXMarkIcon v-else class="w-6 h-6"  @click="changeModelVoice(!sModelVoice)"/>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { Live2DModel, config } from 'pixi-live2d-display/cubism4'
import * as PIXI from 'pixi.js'
import { onBeforeMount, onBeforeUnmount, onMounted, ref } from 'vue'
import { join, resourceDir } from '@tauri-apps/api/path'
import { Store } from 'tauri-plugin-store-api'
import { ArrowPathIcon, SpeakerWaveIcon, SpeakerXMarkIcon } from '@heroicons/vue/16/solid'

//Cubism2.1需要加载live2d.min.js

const live2d_canvas = ref()
const resourceDirPath = await resourceDir()
const path = await join(resourceDirPath, 'data', 'settings.json')

const store = new Store(path)
const modelURL = (await store.get('model_url')) as string
const sModelVoice = ref((await store.get('model_voice')) as boolean)
const sModelScale = ref((await store.get('model_scale')) as number)
const sModelX = ref((await store.get('model_x')) as number)
const sModelY = ref((await store.get('model_y')) as number)
const isHovered = ref(false)
console.log('modelURL:', modelURL)
console.log(store, 12)
var isModelReady = ref(false)

// live2d实例
const model = Live2DModel.fromSync(
  modelURL
  // 'https://cdn.jsdelivr.net/gh/journey-ad/blog-img/live2d/Diana/Diana.model3.json'
  // 'https://cdn.jsdelivr.net/gh/guansss/pixi-live2d-display/test/assets/haru/haru_greeter_t03.model3.json'
)
function reload() {
  location.reload()
}
async function changeModelVoice(openVoice: boolean) {
  sModelVoice.value = openVoice
  config.sound = openVoice
  await store.set('model_voice', sModelVoice.value)
  await store.save()
}
function openSettings() {
  console.log('openSettings')
}
async function ListenEvent() {
  await listen('event_voice', (event: unknown) => {
    console.log(event)
  })
  await listen('event_model_url', (_event: unknown) => {
    reload()
  })
  await listen('event_open_settings', (_event: unknown) => {
    openSettings()
  })
}

onBeforeMount(() => {
  ;(window as any).PIXI = PIXI
})
onMounted(() => {
  console.log('Live2d onMounted')
  ListenEvent()

  // pixi实例
  const app = new PIXI.Application({
    view: live2d_canvas.value,
    autoStart: true,
    resizeTo: window,
    backgroundColor: 0x00ffffff,
    backgroundAlpha: 0,
    useContextAlpha: 'notMultiplied'
  })

  // https://cdn.jsdelivr.net/gh/guansss/pixi-live2d-display/test/assets/haru/haru_greeter_t03.model3.json
  // https://asset.localhost/live2d/buleisite_2/buleisite_2.model3.json
  model.once('ready', () => {
    app.stage.addChild(model)
    model.scale.set(sModelScale.value)
    console.log(model)

    model.x = sModelX.value
    model.y = sModelY.value
    /**
     * 点击live2d人物不同部位时的回调，
     * 只有定义了可点击区域的人物才会收到回调
     */
    model.on('hit', (hitAreas: any) => {
      // hitAreas 包含点击的区域和鼠标坐标

      console.log(hitAreas)
      if (hitAreas.includes('摇头- 身体')) {
        model.motion('Tap害羞-中间刘海')
      }
    })
    /**
     * 让人物做出指定的表情
     * @param name 表情的名称，不传则随机
     */
    model.expression()

    /**
     * 让人物做出指定的动作
     * @param name 动作的名称，不传则随机
     */
    // model.motion()
    isModelReady.value = true
  })
})

onBeforeUnmount(() => {
  console.log('Live2d onBeforeUnmount')
  model.destroy()
  // app.destroy()
})
</script>
