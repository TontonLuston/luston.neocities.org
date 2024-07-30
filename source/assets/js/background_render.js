import * as THREE from 'three';

function getSize() {
    if (window.innerHeight < window.innerWidth) return window.innerHeight - 32;
    return window.innerWidth - 32;
}

var viewSize = getSize();

var rotation = 0;

const scene = new THREE.Scene();

const camera = new THREE.PerspectiveCamera( 55, 1, 0.1, 1000 );

camera.position.z = 5;
camera.rotation.z = 180;
camera.rotation.x = 6.125

const renderer = new THREE.WebGLRenderer();

function hasViewSizeChanged() {
    return viewSize != getSize();
}

function updateViewSize() {
    viewSize = getSize();

    renderer.setSize(viewSize,  viewSize);
}

updateViewSize();

jQuery('#background').html(renderer.domElement);

const geometry = new THREE.CylinderGeometry( 1, 0.8, 5, 8 );
const material = new THREE.MeshBasicMaterial( { wireframe: true } );
const object = new THREE.Mesh( geometry, material );
scene.add( object );

object.rotation.x = 15;

gsap.registerPlugin(ScrollTrigger);

ScrollTrigger.create({
    trigger: "#trigger",
    start: "top top",
    end: "bottom top",
    scrub: true,
    onUpdate: (self) => rotation += self.progress,
});


function animate() {
    if(hasViewSizeChanged()) {
        updateViewSize();
    }

    // object.rotation.x += 0.0050;
    rotation += 0.0025
    object.rotation.y = rotation;
	renderer.render( scene, camera );
}
renderer.setAnimationLoop( animate );
