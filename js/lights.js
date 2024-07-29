const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
camera.position.set(100, 75, 0);
camera.lookAt(0, 0, 0);

const renderer = new THREE.WebGLRenderer({ antialias: true });
const container = document.getElementById('canvas-container');
container.appendChild(renderer.domElement);

// Axes helper
const axesHelper = new THREE.AxesHelper(20);
scene.add(axesHelper);

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
spotlight.target.position.set(0, 0, 0);
scene.add(spotlight);
scene.add(spotlight.target);

// Spotlight Helper
const spotlightHelper = new THREE.SpotLightHelper(spotlight);
scene.add(spotlightHelper);

// GUI setup
const gui = new dat.GUI();
const lightFolder = gui.addFolder('Spotlight');
lightFolder.add(spotlight.target.position, 'y', -50, 50).name('up/down');
lightFolder.add(spotlight, 'intensity', 0, 2).name('Intensity');
lightFolder.add(spotlight, 'angle', 0.05, 0.3).name('Size');
lightFolder.add(spotlight, 'penumbra', 0, 1).name('Focus');
lightFolder.addColor({ color: 0xffffff }, 'color').onChange((color) => {
    spotlight.color = new THREE.Color(color);
});
lightFolder.open();

// Orbit controls - for moving the camera
const controls = new THREE.OrbitControls(camera, renderer.domElement);
controls.enableDamping = true;
controls.dampingFactor = 0.25;
controls.screenSpacePanning = false;

// Angle slider control
var angle = 0;
document.getElementById('slider').addEventListener('input', function(event) {
    angle = parseFloat(event.target.value);
    updateSpotlightAngle(angle);
});

function updateSpotlightAngle(angle) {
    const radians = angle * (Math.PI / 180);
    const radius = 30; 
    spotlight.target.position.x = radius * Math.cos(radians);
    spotlight.target.position.z = radius * Math.sin(radians);
    
    // log the angel to div
    document.getElementById('slider').value = angle;
    document.getElementById('angle_output').innerText = angle;
}

// Button controls
const moveAmount = 1; // Amount to move the spotlight target with each press

document.getElementById('up').addEventListener('mousedown', () => moveSpotlight('up'));
document.getElementById('down').addEventListener('mousedown', () => moveSpotlight('down'));
document.getElementById('left').addEventListener('mousedown', () => moveSpotlight('left'));
document.getElementById('right').addEventListener('mousedown', () => moveSpotlight('right'));

function moveSpotlight(direction) {
    switch(direction) {
        case 'up':
            spotlight.target.position.y += moveAmount;
            break;
        case 'down':
            spotlight.target.position.y -= moveAmount;
            break;
        case 'left':
            angle -= moveAmount;
            break;
        case 'right':
            angle += moveAmount;
            break;
    }
}

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
setSize(); // ensure the initial size is set correctly
