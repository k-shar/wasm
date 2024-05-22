import init, { draw, update_sides } from "../pkg/webassembly_webgl_viewer.js";

const CANVAS_ID = "special";

// TODO: figure out where to store state

async function run() {
  
  await init();

  var i = 0;
  function loop() {
    i += 1;
    draw(CANVAS_ID, i);
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);

}

const shape_sides = document.getElementById("shape_sides");
shape_sides.addEventListener("input", (e) => {
  e.preventDefault();
  update_sides(shape_sides.value);
});

run();
