// Set up the scene
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
camera.position.set(75, 50, 0);
camera.lookAt(0, 0, 0);

const renderer = new THREE.WebGLRenderer({ antialias: true });
const container = document.getElementById('canvas-container');
container.appendChild(renderer.domElement);

// Orbit controls - for moving the camera
const controls = new THREE.OrbitControls(camera, renderer.domElement);
controls.enableDamping = true;
controls.dampingFactor = 0.25;
controls.screenSpacePanning = false;
controls.maxPolarAngle = Math.PI / 2;

// Axes helper
const axesHelper = new THREE.AxesHelper(20);
scene.add(axesHelper);

// Floor
const floorGeo = new THREE.PlaneGeometry(1000, 1000);
const floorMat = new THREE.MeshStandardMaterial({ color: 0xF08080, side: THREE.DoubleSide });
const floor = new THREE.Mesh(floorGeo, floorMat);
floor.rotation.x = -Math.PI / 2;
floor.position.y = -0.1;
scene.add(floor);

// Ground
const groundGeometry = new THREE.PlaneGeometry(100, 100);
const groundMaterial = new THREE.MeshStandardMaterial({ color: 0x808080, side: THREE.DoubleSide });
const ground = new THREE.Mesh(groundGeometry, groundMaterial);
ground.rotation.x = -Math.PI / 2;
scene.add(ground);

// Ambient light
const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
scene.add(ambientLight);

// Create multiple spotlights
const lights = [];

for (let i = 0; i < 3; i++) {
    const spotlight = new THREE.SpotLight(0xffffff);
    spotlight.position.set(-50 + i * 50, 50, 50);
    spotlight.distance = 0; // infinite throw
    spotlight.angle = 0.1; // how wide the beam is
    spotlight.penumbra = 0.6;
    spotlight.target.position.set(0, 30, 0);
    scene.add(spotlight);
    scene.add(spotlight.target);

    const spotlightHelper = new THREE.SpotLightHelper(spotlight);
    scene.add(spotlightHelper);

    lights.push({
        spotlight: spotlight,
        helper: spotlightHelper
    });
}

// Store the index of the selected light
let selectedLightIndex = 0;

// Handle light selection
document.getElementById('light-selector').addEventListener('change', function(event) {
    selectedLightIndex = parseInt(event.target.value);
    updateControls();
});

// Function to update controls based on the selected light
function updateControls() {
    const selectedLight = lights[selectedLightIndex].spotlight;
    document.getElementById('intensity').value = selectedLight.intensity * 10;
    document.getElementById('size').value = selectedLight.angle * 100;
    document.getElementById('focus').value = selectedLight.penumbra * 100;
    document.getElementById('up_down').value = selectedLight.target.position.y;
    const lightPosition = lights[selectedLightIndex].spotlight.position;
    const angle = Math.atan2(lightPosition.z - selectedLight.target.position.z, lightPosition.x - selectedLight.target.position.x) * (180 / Math.PI);
    document.getElementById('left_right').value = angle;
}

// Initial call to set controls
updateControls();

// LEFT RIGHT
document.getElementById('left_right').addEventListener('input', function(event) {
    const angle = parseFloat(event.target.value);
    updateLightPosition('left_right', angle);
});

// UP DOWN
document.getElementById('up_down').addEventListener('input', function(event) {
    const y = parseFloat(event.target.value);
    lights[selectedLightIndex].spotlight.target.position.y = y;
});

// Intensity
document.getElementById('intensity').addEventListener('input', function(event) {
    const intensity = parseFloat(event.target.value) / 10;
    lights[selectedLightIndex].spotlight.intensity = intensity;
});

// Size
document.getElementById('size').addEventListener('input', function(event) {
    const size = parseFloat(event.target.value) / 100;
    lights[selectedLightIndex].spotlight.angle = size;
});

// Focus
document.getElementById('focus').addEventListener('input', function(event) {
    const focus = parseFloat(event.target.value) / 100;
    lights[selectedLightIndex].spotlight.penumbra = focus;
});

// Update light position
function updateLightPosition(controlId, value) {
    const angle = parseFloat(value);
    const radians = angle * (Math.PI / 180);
    const radius = 30;
    const light = lights[selectedLightIndex].spotlight;
    light.target.position.x = radius * Math.cos(radians) + light.position.x;
    light.target.position.z = radius * Math.sin(radians) + light.position.z;
}

// Color picker setup
const colorPicker = new iro.ColorPicker("#picker", {
    width: 150,
    color: "#ffffff"
});

colorPicker.on(["color:init", "color:change"], function(color){
    lights[selectedLightIndex].spotlight.color = new THREE.Color(color.hexString);
});

// Animation loop
function animate() {
    requestAnimationFrame(animate);
    lights.forEach(light => light.helper.update());
    renderer.render(scene, camera);
}
animate();

// Resize handling
function setSize() {
    const width = container.clientWidth;
    const height = container.clientHeight;
    renderer.setSize(width, height);
    camera.aspect = width / height;
    camera.updateProjectionMatrix();
}

window.addEventListener('resize', () => {
    setSize();
});
setSize();
