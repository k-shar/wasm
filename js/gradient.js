import init, { gradient_draw, g_update_sides } from "../pkg/webassembly_webgl_viewer.js";

const CANVAS_ID = "gradient";

async function run() {
  
  await init();
  document.getElementById("shape_sides").value = 10;

  function loop() {
    gradient_draw(CANVAS_ID);
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);

}

const shape_sides = document.getElementById("shape_sides");
shape_sides.addEventListener("input", (e) => {
  e.preventDefault();
  g_update_sides(shape_sides.value);
  document.getElementById("shape_sides_text").innerText = "sides: " + shape_sides.value;
});


run();