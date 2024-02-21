import {
  Color4,
  Engine,
  Scene,
  Vector3,
  ArcRotateCamera,
  HemisphericLight,
  Color3,
  DirectionalLight,
  ShadowGenerator,
  SceneLoader,
  HavokPlugin,
  Mesh,
  StandardMaterial,
  MirrorTexture,
  Plane,
  MeshBuilder,
  DefaultRenderingPipeline,
  ImageProcessingConfiguration,
  Material
} from '@babylonjs/core'

import * as GUI from '@babylonjs/gui'
import type { ISceneBuilder, MMDOptions } from './MMDRuntime'
import { BvmdLoader, MmdCamera, MmdPlayerControl, MmdRuntime, StreamAudioPlayer } from 'babylon-mmd'
export class SceneBuilder implements ISceneBuilder {
  private _options: MMDOptions = {
    mmdUrl: 'mmdUrl',
    mmdCamera: true,
    paused: false,
    volume: 0.8
  }
  public async build(canvas: HTMLCanvasElement, engine: Engine, mmdOptions: MMDOptions): Promise<Scene> {
    console.log('SceneBuilder build')
    this._options = mmdOptions
    const mmdUrl = this._options.mmdUrl

    // 加载mmd设置
    // https://doc.babylonjs.com/communityExtensions/mmdLoader
    // https://noname0310.github.io/babylon-mmd/docs/quick-start/create-basic-scene/
    const settings = await fetch(mmdUrl).then(response => response.json())
    console.log('sets: ', settings)

    const pmxLoader = SceneLoader.GetPluginForExtension('.pmx') as any
    const materialBuilder = pmxLoader.materialBuilder
    materialBuilder.useAlphaEvaluation = false
    materialBuilder.loadOutlineRenderingProperties = (): void => {
      /* do nothing */
    }
    const alphaBlendMaterials = ['face02', 'Facial02', 'HL', 'Hairshadow', 'q302']
    const alphaTestMaterials = ['q301']
    materialBuilder.afterBuildSingleMaterial = (material: any) => {
      if (!alphaBlendMaterials.includes(material.name) && !alphaTestMaterials.includes(material.name)) return
      material.transparencyMode = alphaBlendMaterials.includes(material.name) ? Material.MATERIAL_ALPHABLEND : Material.MATERIAL_ALPHATEST
      material.useAlphaFromDiffuseTexture = true
      material.diffuseTexture.hasAlpha = true
    }

    const scene = new Scene(engine)
    scene.clearColor = new Color4(0.95, 0.95, 0.95, 1.0)
    const mmdCamera = new MmdCamera('mmdCamera', new Vector3(0, 10, 0), scene)
    mmdCamera.maxZ = 500
    mmdCamera.minZ = 0.1

    const camera = new ArcRotateCamera('arcRotateCamera', 0, 0, 45, new Vector3(0, 0, 0), scene)
    camera.maxZ = 500
    camera.setPosition(new Vector3(0, 10, -45))
    camera.attachControl(canvas, true)
    camera.inertia = 0.8
    camera.speed = 10

    const hemisphericLight = new HemisphericLight('hemisphericLight', new Vector3(0, 1, 0), scene)
    hemisphericLight.intensity = 0.4
    hemisphericLight.specular = new Color3(0, 0, 0)
    hemisphericLight.groundColor = new Color3(1, 1, 1)

    const directionalLight = new DirectionalLight('DirectionalLight', new Vector3(0.5, -1, 1), scene)
    directionalLight.intensity = 0.8
    directionalLight.autoCalcShadowZBounds = false
    directionalLight.autoUpdateExtends = false
    directionalLight.shadowMaxZ = 20
    directionalLight.shadowMinZ = -15
    directionalLight.orthoTop = 18
    directionalLight.orthoBottom = -1
    directionalLight.orthoLeft = -10
    directionalLight.orthoRight = 10
    directionalLight.shadowOrthoScale = 0

    const shadowGenerator = new ShadowGenerator(1024, directionalLight, true)
    shadowGenerator.usePercentageCloserFiltering = true
    shadowGenerator.forceBackFacesOnly = true
    shadowGenerator.filteringQuality = ShadowGenerator.QUALITY_MEDIUM
    shadowGenerator.frustumEdgeFalloff = 0.1

    // 启用物理效果
    // const mmdRuntime = new MmdRuntime(scene, new MmdPhysics(scene))
    const mmdRuntime = new MmdRuntime(scene)
    mmdRuntime.register(scene)

    // 音频
    const audioPlayer = new StreamAudioPlayer(scene)
    audioPlayer.preservesPitch = false
    audioPlayer.source = 'https://noname0310.github.io/web-mmd-viewer/melancholic_night/mmd_public/motion/melancholy_night/melancholy_night.mp3'
    mmdRuntime.setAudioPlayer(audioPlayer)
    mmdRuntime.playAnimation()

    const mmdPlayerControl = new MmdPlayerControl(scene, mmdRuntime, audioPlayer)
    mmdPlayerControl.showPlayerControl()

    engine.displayLoadingUI()

    let loadingTexts: string[] = []
    const updateLoadingText = (updateIndex: number, text: string): void => {
      loadingTexts[updateIndex] = text
      engine.loadingUIText = '<br/><br/><br/><br/>' + loadingTexts.join('<br/><br/>')
    }

    const promises: Promise<any>[] = []
    // 加载 vmd 动作
    const bvmdLoader = new BvmdLoader(scene)

    promises.push(
      bvmdLoader.loadAsync('motion', 'https://noname0310.github.io/web-mmd-viewer/melancholic_night/mmd_public/motion/melancholy_night/motion.bvmd', event =>
        updateLoadingText(0, `Loading motion... ${event.loaded}/${event.total} (${Math.floor((event.loaded * 100) / event.total)}%)`)
      )
    )

    // 加载 mmd 模型
    promises.push(
      SceneLoader.ImportMeshAsync(
        undefined,
        'https://noname0310.github.io/web-mmd-viewer/melancholic_night/mmd_public/model/yyb_hatsune_miku_10th_ff/yyb_hatsune_miku_10th_v1.02.pmx',
        undefined,
        scene,
        event => updateLoadingText(1, `Loading model... ${event.loaded}/${event.total} (${Math.floor((event.loaded * 100) / event.total)}%)`)
      )
    )

    // promises.push(
    //   (async () => {
    //     updateLoadingText(2, 'Loading physics engine...')
    //     const havokPlugin = new HavokPlugin()
    //     scene.enablePhysics(new Vector3(0, -9.8, 0), havokPlugin)
    //     updateLoadingText(2, 'Loading physics engine... Done')
    //   })()
    // )

    loadingTexts = new Array(promises.length).fill('')
    const loadResults = await Promise.all(promises)

    scene.onAfterRenderObservable.addOnce(() => engine.hideLoadingUI())

    mmdRuntime.setCamera(mmdCamera)
    mmdCamera.addAnimation(loadResults[0])
    mmdCamera.setAnimation('motion')

    const modelMesh = loadResults[1].meshes[0] as Mesh
    modelMesh.receiveShadows = true
    shadowGenerator.addShadowCaster(modelMesh)
    const mmdModel = mmdRuntime.createMmdModel(modelMesh)
    mmdModel.addAnimation(loadResults[0])
    mmdModel.setAnimation('motion')

    // const bodyBone = modelMesh.skeleton!.bones.find(bone => bone.name === 'センター')
    // scene.onBeforeRenderObservable.add(() => {
    //   bodyBone!.getFinalMatrix().getTranslationToRef(directionalLight.position)
    //   directionalLight.position.y -= 10
    // })

    const ground = MeshBuilder.CreateGround('Ground', { width: 100, height: 100, subdivisions: 2, updatable: false }, scene)
    ground.receiveShadows = true
    const groundMaterial = (ground.material = new StandardMaterial('GroundMaterial', scene))
    groundMaterial.diffuseColor = new Color3(0.65, 0.65, 0.65)
    groundMaterial.specularPower = 128
    const groundReflectionTexture = (groundMaterial.reflectionTexture = new MirrorTexture('MirrorTexture', 1024, scene, true))
    groundReflectionTexture.mirrorPlane = Plane.FromPositionAndNormal(ground.position, ground.getFacetNormal(0).scale(-1))
    groundReflectionTexture.renderList = [modelMesh]
    groundReflectionTexture.level = 0.45

    const defaultPipeline = new DefaultRenderingPipeline('default', true, scene, [mmdCamera, camera])
    defaultPipeline.samples = 4
    defaultPipeline.bloomEnabled = true
    defaultPipeline.chromaticAberrationEnabled = true
    defaultPipeline.chromaticAberration.aberrationAmount = 1
    defaultPipeline.fxaaEnabled = true
    defaultPipeline.imageProcessingEnabled = true
    defaultPipeline.imageProcessing.toneMappingEnabled = true
    defaultPipeline.imageProcessing.toneMappingType = ImageProcessingConfiguration.TONEMAPPING_ACES
    defaultPipeline.imageProcessing.vignetteWeight = 0.5
    defaultPipeline.imageProcessing.vignetteStretch = 0.5
    defaultPipeline.imageProcessing.vignetteColor = new Color4(0, 0, 0, 0)
    defaultPipeline.imageProcessing.vignetteEnabled = true

    const guiCamera = new ArcRotateCamera('GUICamera', Math.PI / 2 + Math.PI / 7, Math.PI / 2, 100, new Vector3(0, 20, 0), scene)
    guiCamera.layerMask = 0x10000000
    scene.activeCameras = [mmdCamera, guiCamera]

    let lastClickTime = -Infinity
    canvas.onclick = (): void => {
      const currentTime = performance.now()
      if (500 < currentTime - lastClickTime) {
        lastClickTime = currentTime
        return
      }
      lastClickTime = -Infinity

      if (scene.activeCameras?.[0] === mmdCamera) scene.activeCameras = [camera, guiCamera]
      else scene.activeCameras = [mmdCamera, guiCamera]
    }

    const advancedTexture = GUI.AdvancedDynamicTexture.CreateFullscreenUI('UI')
    advancedTexture.layer!.layerMask = 0x10000000
    const textblock = new GUI.TextBlock()
    textblock.widthInPixels = 500
    textblock.heightInPixels = 110
    textblock.left = 10
    textblock.text = 'メランコリ・ナイト / melancholy night feat.初音ミク\n\nMusic & Lyrics by higma\nMotion by ほうき堂\nModel: YYB Hatsune Miku 10th by YYB'
    textblock.fontSize = 16

    textblock.textHorizontalAlignment = GUI.Control.HORIZONTAL_ALIGNMENT_LEFT
    textblock.horizontalAlignment = GUI.Control.HORIZONTAL_ALIGNMENT_LEFT
    textblock.verticalAlignment = GUI.Control.VERTICAL_ALIGNMENT_TOP
    textblock.color = 'black'
    advancedTexture.addControl(textblock)

    return scene
  }
}
