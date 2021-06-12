import init, { greet } from "../pkg/wasm_canvas.js";

async function run() {
  const wasm = await fetch("../public/pkg/wasm_canvas_bg.wasm");
  await init(wasm);
  greet("nabezokodaikon");
}

run();