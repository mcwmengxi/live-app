import { Engine, Scene, Vector3 } from "@babylonjs/core";
import type { ISceneBuilder, MMDOptions } from "./MMDRuntime";
import { MmdCamera } from 'babylon-mmd'
export class SceneBuilder implements ISceneBuilder {
  private _options: MMDOptions = {
    mmdUrl: 'mmdUrl',
    mmdCamera: true,
    paused: false,
    volume: 0.8
  }
  public async build(canvas: HTMLCanvasElement, engine: Engine, mmdOptions: MMDOptions): Promise<Scene> {
    console.log("SceneBuilder build");
    this._options = mmdOptions;
    const mmdUrl = this._options.mmdUrl
    
    // 加载mmd设置
    // https://doc.babylonjs.com/communityExtensions/mmdLoader
    // https://noname0310.github.io/babylon-mmd/docs/quick-start/create-basic-scene/
    const settings = await fetch(mmdUrl).then(response => response.json());
    console.log("sets: ", settings);
    const scene = new Scene(engine)
    const mmdCamera =new MmdCamera("mmdCamera", new Vector3(0, 10, 0), scene)
    mmdCamera.maxZ = 500;
    mmdCamera.minZ = 0.1;

    mmdCamera.attachControl(canvas, true);
    return scene;
  }
}