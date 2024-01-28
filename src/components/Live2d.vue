<template>
  <div class="w-full h-full border border-gray-100 border-dashed static">
    <canvas ref="live2d_canvas" class="w-full h-full"></canvas>
    <div
      class="absolute inset-y-0 flex flex-col right-0 mx-4 my-4 py-4 px-2 space-y-4 backdrop-blur-3xl bg-white/30"
    >
      <ul v-if="isModelReady" data-tauri-drag-region>
        <li class="w-8 h-8" @click="reload">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-6 h-6"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
            />
          </svg>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { Live2DModel } from 'pixi-live2d-display/cubism4'
import * as PIXI from 'pixi.js'
import { onBeforeMount, onBeforeUnmount, onMounted, ref } from 'vue'
import { join, resourceDir } from '@tauri-apps/api/path'
import { Store } from 'tauri-plugin-store-api'

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
