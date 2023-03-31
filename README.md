## Quick Start

0. Change the Bevy installation from `path = { path = ...` to the git version `# bevy = {git = ...`. The correct git version is commented out in `Cargo.toml`.
1. Install `trunk` and `simple-http-server` (or a http server of your choice). [Trunk linked here.](https://trunkrs.dev/)
2. Run `trunk build` to build WASM files + HTML + javascript to the `/dist` directory.
3. Replace the `dropObject` function in the generated javascript with:

```js
const webgpu_classes = Object.getOwnPropertyNames(window)
    .filter(k => k.startsWith("GPU") && typeof window[k] === 'function')
    .map(k => k);

const is_webgpu_obj = o => o && o.constructor && webgpu_classes.some(webgpu_class => o.constructor.name === webgpu_class);

function dropObject(idx) {
    if (idx < 132) return;
    if (is_webgpu_obj(heap[idx])) return;
    heap[idx] = heap_next;
    heap_next = idx;
}
```
(see https://github.com/gfx-rs/wgpu/issues/3430 for details)

4. Run `simple-http-server dist` to start the webserver.
5. Navigate to `localhost:8000` in either Chrome Canary or Firefox nightly, with the respective WebGPU flags set.

