import init, { point_draw, p_update_resolution } from "../pkg/webassembly_webgl_viewer.js";

const CANVAS_ID = "point";

async function run() {
  
  await init();
  
  document.getElementById("shape_sides").value = 10;

  function loop() {
    point_draw(CANVAS_ID);
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);

}

const shape_sides = document.getElementById("shape_sides");
shape_sides.addEventListener("input", (e) => {
  e.preventDefault();
  p_update_resolution(shape_sides.value);
  document.getElementById("shape_sides_text").innerText = "resolution: " + shape_sides.value;
});

run();