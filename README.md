# Tauri + Vue 3 + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).


### 配置文件

live2d资源里有一种.exp3.json， 这些文件是不同的表情文件，里面的内容实际上就是改变人物的不同参数，让人物做出对应的样子。
有可能设计师输出的.model3.json文件没有把这些表情文件加入到配置中，需要自己配置一下

```
{
    Version: 3,
    FileReferences: {
        Moc: "xxx.moc3",
        Textures: [
            "xxx.8192/texture_00.png"
        ],
        Physics: "xxx.physics3.json",
        DisplayInfo: "xxx.cdi3.json",

        // 这个字段，代表所有的表情，如果原本没有到话，可以自己添加上
        Expressions: [
            {
                Name: "开心", // 表情的名称，调用model.expression时的入参
                File: "开心.exp3.json"
            }
        ]
    }
} 
```