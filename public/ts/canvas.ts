import init, {
  console_log,
  timer,
} from "../pkg/wasm_canvas.js";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))
// function sleep(ms: number) {
  // return new Promise(resolve => setTimeout(resolve, ms))
// }

// async function sleep(ms: number) {
  // await setTimeout(ms);
// }

// function sleep(ms: number): Promise<void> {
  // return new Promise<void>(resolve => setTimeout(resolve, ms))
// }

async function run() {
  const wasm = await fetch("public/pkg/wasm_canvas_bg.wasm");
  await init(wasm);
  console_log("Start!");
  timer(10);
}

run();
