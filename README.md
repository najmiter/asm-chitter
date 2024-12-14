# Getting started

```bash
cargo install wasm-pack
```

```bash
wasm-pack build --target web
```

```bash
npm init wasm-app my-web-app
```

Then in the `index.js` of this template, you may start using the wasm stuff:

```js
import init, { chittify } from '../pkg/chitter.js';

await init(); // MAKE SURE TO INITIALIZE!
console.log(chittify('some string'));
```
