import init, { sin_draw, s_update_resolution, s_update_wavelength } from "../pkg/webassembly_webgl_viewer.js";

const CANVAS_ID = "sin_wave";

async function run() {
  
  await init();
  
  document.getElementById("wavelength").value = 10;

//   function loop() {
//     sin_draw(CANVAS_ID);
//     requestAnimationFrame(loop);
//   }
//   requestAnimationFrame(loop);

}

const resolution = document.getElementById("resolution");
resolution.addEventListener("input", (e) => {
  e.preventDefault();
  s_update_resolution(resolution.value);
  document.getElementById("resolution_text").innerText = "resolution: " + resolution.value;
  sin_draw(CANVAS_ID);
});

const wavelength = document.getElementById("wavelength");
wavelength.addEventListener("input", (e) => {
  e.preventDefault();
  s_update_wavelength(wavelength.value);
  document.getElementById("wavelength_text").innerText = "wavelength: " + wavelength.value;
  sin_draw(CANVAS_ID);
});

run();
