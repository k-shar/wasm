


const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
camera.position.set(75, 50, 0);
camera.lookAt(0, 0, 0);

const renderer = new THREE.WebGLRenderer({ antialias: true });
const container = document.getElementById('canvas-container');
container.appendChild(renderer.domElement);

// Axes helper
const axesHelper = new THREE.AxesHelper(20);
scene.add(axesHelper);

// Floor 
const floor_geo = new THREE.PlaneGeometry(1000, 1000);
const floor_mat = new THREE.MeshStandardMaterial({ color: 0xF08080, side: THREE.DoubleSide });
const floor = new THREE.Mesh(floor_geo, floor_mat);
floor.rotation.x = -Math.PI / 2;
floor.position.y = -0.1;
scene.add(floor);

// Ground setup
const groundGeometry = new THREE.PlaneGeometry(100, 100);
const groundMaterial = new THREE.MeshStandardMaterial({ color: 0x808080, side: THREE.DoubleSide });
const ground = new THREE.Mesh(groundGeometry, groundMaterial);
ground.rotation.x = -Math.PI / 2;
scene.add(ground);

// Ambient light
const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
scene.add(ambientLight);

// Spotlight setup
const spotlight = new THREE.SpotLight(0xffffff);
spotlight.position.set(-50, 50, 50);
spotlight.distance = 0; // infinite throw
spotlight.angle = 0.1; // how wide the beam is
spotlight.penumbra = 0.6;
spotlight.target.position.set(0, 30, 0);
scene.add(spotlight);
scene.add(spotlight.target);

// Spotlight Helper
const spotlightHelper = new THREE.SpotLightHelper(spotlight);
scene.add(spotlightHelper);


// Orbit controls - for moving the camera
const controls = new THREE.OrbitControls(camera, renderer.domElement);
controls.enableDamping = true;
controls.dampingFactor = 0.25;
controls.screenSpacePanning = false;
controls.maxPolarAngle = Math.PI / 2;

// LEFT RIGHT
var angle = -45;
document.getElementById('left_right').addEventListener('input', function(event) {
    angle = parseFloat(event.target.value);
    updateSpotlightAngle(angle);
});
function updateSpotlightAngle(angle) {
    const radians = angle * (Math.PI / 180);
    const radius = 30; 
    spotlight.target.position.x = radius * Math.cos(radians) + spotlight.position.x;
    spotlight.target.position.z = radius * Math.sin(radians) + spotlight.position.z;
    document.getElementById('left_right').value = angle;
}

// COLOUR
var colorPicker = new iro.ColorPicker("#picker", {
    width: 150,
    color: "#ffffff"
});
  
colorPicker.on(["color:init", "color:change"], function(color){
  spotlight.color = new THREE.Color(color.hexString);
});

// Animation loop
function animate() {
    requestAnimationFrame(animate);
    spotlightHelper.update();
    updateSpotlightAngle(angle);
    renderer.render(scene, camera);
    console.log(angle);
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
