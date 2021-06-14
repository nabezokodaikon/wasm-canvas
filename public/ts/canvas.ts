import init, {
  console_log,
  timer,
} from "../pkg/wasm_canvas.js";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))

async function run() {
  const wasm = await fetch("public/pkg/wasm_canvas_bg.wasm");
  await init(wasm);
  console_log("Start!");
  timer(10);
}

run();
