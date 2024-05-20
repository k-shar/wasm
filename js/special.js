import init, { special } from "../pkg/webassembly_webgl_viewer.js";

const CANVAS_ID = "special";

async function run() {
  
  await init();

  var i = 0;
  function loop() {
    i += 1;
    special(CANVAS_ID, i);
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);

}

run();
