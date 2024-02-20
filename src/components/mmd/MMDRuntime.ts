import { Engine, Scene } from "@babylonjs/core";

export interface ISceneBuilder {
  build(canvas: HTMLCanvasElement, engine: Engine, mmdOptions: MMDOptions): Scene | Promise<Scene>;
}
export interface BaseRuntimeInitParams {
  canvas: HTMLCanvasElement;
  engine: Engine;
  sceneBuilder: ISceneBuilder;
  mmdOptions: MMDOptions;
}
export interface MMDOptions {
  mmdUrl: string;
  mmdCamera: boolean;
  paused: boolean;
  volume: number;
}
export class BaseRuntime {
  private readonly _canvas: HTMLCanvasElement;
  private readonly _engine: Engine;
  private readonly _mmdOptions: MMDOptions;
  private _scene: Scene;
  private _onTick: () => void;

  private constructor(params: BaseRuntimeInitParams) {
    this._canvas = params.canvas;
    this._engine = params.engine;
    this._mmdOptions = params.mmdOptions;
    console.log("this._canvas.height ", this._canvas.height);
    this._scene = null!;
    this._onTick = null!;
  }
  public static async Create(params: BaseRuntimeInitParams){
    const runtime = new BaseRuntime(params);
    runtime._scene = await runtime._initialize(params.sceneBuilder);
    runtime._onTick = runtime._makeOnTick();

    return runtime;
  }

  private async _initialize(sceneBuilder: ISceneBuilder): Promise<Scene> {
    return await sceneBuilder.build(this._canvas, this._engine, this._mmdOptions);
  }

  private _makeOnTick():()=>void {
    const scene = this._scene;
    const canvas = this._canvas;
    console.log("scene.clearColor", JSON.stringify(scene.clearColor));
    console.log("canvas.height ", canvas.height);
    return () => scene.render();
  }
  public run(): void {
    window.addEventListener("resize", this._onResize);
    this._engine.runRenderLoop(this._onTick);
  }

  public dispose(): void {
    window.removeEventListener("resize", this._onResize);
    this._engine.dispose();
  }
  
  private readonly _onResize = (): void => {
    this._engine.resize();
  };
}