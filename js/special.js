import init, { draw, update_sides, update_rotation_speed, update_colour_speed } from "../pkg/webassembly_webgl_viewer.js";

const CANVAS_ID = "special";

async function run() {
  
  await init();
  document.getElementById("shape_sides").value = 10;
  document.getElementById("rotation_speed").value = 50;

  function loop() {
    draw(CANVAS_ID);
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);

}

const shape_sides = document.getElementById("shape_sides");
shape_sides.addEventListener("input", (e) => {
  e.preventDefault();
  update_sides(shape_sides.value);
  document.getElementById("shape_sides_text").innerText = "sides: " + shape_sides.value;
});

const rotation_speed = document.getElementById("rotation_speed");
rotation_speed.addEventListener("input", (e) => {
  e.preventDefault();
  update_rotation_speed(rotation_speed.value);
  document.getElementById("rotation_speed_text").innerText = "rotation speed: " + rotation_speed.value + "%";
});

const colour_speed = document.getElementById("colour_speed");
colour_speed.addEventListener("input", (e) => {
  e.preventDefault();
  update_colour_speed(colour_speed.value);
  document.getElementById("colour_speed_text").innerText = "colour speed: " + colour_speed.value + "%";
});

run();
