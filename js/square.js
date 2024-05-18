import init, { draw_square } from "../pkg/webassembly_webgl_viewer.js";

const CANVAS_ID = "square";

async function run() {
  await init();
  const color = [0.0, 0.5, 0.9, 1.0];
  draw_square(CANVAS_ID, color);
}

run();

const colorChangerForm = document.getElementById("red");
colorChangerForm.addEventListener("input", (e) => {
  
  console.log(e);

  const color = [
    clampRGBValue(e.target.elements.red.value),
    clampRGBValue(e.target.elements.green.value),
    clampRGBValue(e.target.elements.blue.value),
    1.0,
  ];

  draw_square(CANVAS_ID, color);
});

function clampRGBValue(value) {
  return parseFloat((parseFloat(value) / 255 || 0).toFixed(2));
}
