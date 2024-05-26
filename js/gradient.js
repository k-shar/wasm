import init, { gradient_draw } from "../pkg/webassembly_webgl_viewer.js";

const CANVAS_ID = "gradient";

async function run() {
  
  await init();

  function loop() {
    gradient_draw(CANVAS_ID);
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);

}


run();