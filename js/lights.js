// Set up the scene
const scene = new THREE.Scene();
const camera = createCamera();
const renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.shadowMap.enabled = true; // Enable shadow maps in the renderer
const container = document.getElementById('canvas-container');
container.appendChild(renderer.domElement);
const controls = createControls();
const lights = createLights();
var selectedLightIndex = 0;
const colorPicker = createColorPicker();

// Initialize the scene
function createCamera() {
    const cam = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    cam.position.set(75, 50, 0);
    cam.lookAt(0, 0, 0);
    return cam;
}

function createControls() {
    const ctrl = new THREE.OrbitControls(camera, renderer.domElement);
    ctrl.enableDamping = true;
    ctrl.dampingFactor = 0.25;
    ctrl.screenSpacePanning = false;
    ctrl.maxPolarAngle = Math.PI / 2;
    return ctrl;
}


function createColorPicker() {
    var colorPicker = new iro.ColorPicker("#picker", {
        width: 150,
        color: "#ffffff"
    });
    
    colorPicker.on(["color:init", "color:change"], function(color){
        lights[selectedLightIndex].spotlight.color.set(new THREE.Color(color.hexString));
    });
}

function createLights() {
    const lightsArr = [];
    const numberOfLights = 3;
    for (let i = 0; i < numberOfLights; i++) {
        const spotlight = new THREE.SpotLight(0xffffff);
        spotlight.position.set(-50, 50, -50 + i * (100 / (numberOfLights-1)));
        // spotlight.target.position.set(0, 0, 25 - i * (50 / (numberOfLights-1)));
        spotlight.target.position.set(0, 0, 0);
        spotlight.distance = 0; // infinite throw
        spotlight.angle = 0.1; // how wide the beam is
        spotlight.penumbra = 0.6; // how blurry the beam is
        spotlight.castShadow = true;

        scene.add(spotlight);
        scene.add(spotlight.target);
        
        const spotlightHelper = new THREE.SpotLightHelper(spotlight);
        scene.add(spotlightHelper);
        
        lightsArr.push({ spotlight, helper: spotlightHelper });
    }

    return lightsArr;
}

// Add initial scene objects
function initScene() {
    addFloor();
    addGround();
    addAmbientLight();
    const cube = addCube();
    setupEventListeners();
    updateControls();

    // Add DragControls
    const dragControls = new THREE.DragControls([cube], camera, renderer.domElement);
    dragControls.addEventListener('dragstart', function(event) {
        controls.enabled = false;
    });
    dragControls.addEventListener('dragend', function(event) {
        controls.enabled = true;
    });

}

function addCube() {
    const geometry = new THREE.BoxGeometry();
    const material = new THREE.MeshStandardMaterial({ color: 0xf00ff0 });
    const cube = new THREE.Mesh(geometry, material);
    cube.scale.set(5, 15, 7);
    cube.position.set(25, 5, 0);
    cube.castShadow = true;
    scene.add(cube);
    return cube;
}

function addFloor() {
    const floorGeo = new THREE.PlaneGeometry(1000, 1000);
    const floorMat = new THREE.MeshStandardMaterial({ color: 0x80F080, side: THREE.DoubleSide });
    const floor = new THREE.Mesh(floorGeo, floorMat);
    floor.rotation.x = -Math.PI / 2;
    floor.position.y = -0.1;
    floor.receiveShadow = true;
    scene.add(floor);
}

function addGround() {
    const groundGeometry = new THREE.PlaneGeometry(100, 100);
    const groundMaterial = new THREE.MeshStandardMaterial({ color: 0x808080, side: THREE.DoubleSide });
    const ground = new THREE.Mesh(groundGeometry, groundMaterial);
    ground.rotation.x = -Math.PI / 2;
    ground.receiveShadow = true;
    scene.add(ground);
}

function addAmbientLight() {
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);
}

// Event listeners
function setupEventListeners() {
    

    document.getElementById('light-selector').addEventListener('change', function(event) {
        selectedLightIndex = parseInt(event.target.value);
        updateControls();
    });

    document.getElementById('left_right').addEventListener('input', function(event) {
        const angle = parseFloat(event.target.value);
        angle_to_xy(angle);
    });

    document.getElementById('up_down').addEventListener('input', function(event) {
        const y = parseFloat(event.target.value);
        lights[selectedLightIndex].spotlight.target.position.y = y;
    });

    document.getElementById('intensity').addEventListener('input', function(event) {
        const intensity = parseFloat(event.target.value) / 10;
        lights[selectedLightIndex].spotlight.intensity = intensity;
    });

    document.getElementById('size').addEventListener('input', function(event) {
        const size = parseFloat(event.target.value) / 100;
        lights[selectedLightIndex].spotlight.angle = size;
    });

    document.getElementById('focus').addEventListener('input', function(event) {
        const focus = parseFloat(event.target.value) / 100;
        lights[selectedLightIndex].spotlight.penumbra = focus;
    });

    window.addEventListener('resize', () => setSize());
}

// Update controls
function updateControls() {
    const selectedLight = lights[selectedLightIndex].spotlight;
    document.getElementById('intensity').value = selectedLight.intensity * 10;
    document.getElementById('size').value = selectedLight.angle * 100;
    document.getElementById('focus').value = selectedLight.penumbra * 100;
    document.getElementById('up_down').value = selectedLight.target.position.y;
    const lightPosition = selectedLight.position;
    const angle = Math.atan2(lightPosition.z - selectedLight.target.position.z, lightPosition.x - selectedLight.target.position.x) * (180 / Math.PI) + 180;
    console.log(angle);
    document.getElementById('left_right').value = angle;
    // document.getElementById('lef').innerText = angle;
}

function angle_to_xy(value) {
    const angle = parseFloat(value);
    const radians = angle * (Math.PI / 180);
    const radius = 30;
    const light = lights[selectedLightIndex].spotlight;
    light.target.position.x = radius * Math.cos(radians) + light.position.x;
    light.target.position.z = radius * Math.sin(radians) + light.position.z;
}


// Animation loop
function animate() {
    requestAnimationFrame(animate);
    renderer.render(scene, camera);
    lights.forEach(light => light.helper.update());
    if (document.getElementById('help_lines').checked) {
        
    }
}

function setSize() {
    const width = container.clientWidth;
    const height = container.clientHeight;
    renderer.setSize(width, height);
    camera.aspect = width / height;
    camera.updateProjectionMatrix();
}

// Initialize everything
initScene();
animate();
setSize();
updateControls();