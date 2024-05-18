import init, { draw_square } from "../pkg/webassembly_webgl_viewer.js";

const CANVAS_ID = "square";

async function run() {
  await init();
  const color = [0.0, 0.5, 0.9, 1.0];
  draw_square(CANVAS_ID, color);
}

run();

const colorChangerForm = document.getElementById("color-changer");
colorChangerForm.addEventListener("input", (e) => {

  e.preventDefault();

  const color = [
    clampRGBValue(document.getElementById("red").value),
    clampRGBValue(document.getElementById("green").value),
    clampRGBValue(document.getElementById("blue").value),
    1.0,
  ];

  draw_square(CANVAS_ID, color);
});

function clampRGBValue(value) {
  return parseFloat((parseFloat(value) / 255 || 0).toFixed(2));
}
