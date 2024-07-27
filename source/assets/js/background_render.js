import * as THREE from 'three';

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera( 55, window.innerWidth / window.innerHeight, 0.1, 1000 );

const renderer = new THREE.WebGLRenderer();
renderer.setSize( window.innerWidth, window.innerHeight );
jQuery('#background').html(renderer.domElement);

const geometry = new THREE.CylinderGeometry( 1, 1, 5, 8 );
const material = new THREE.MeshBasicMaterial( { wireframe: true } );
const object = new THREE.Mesh( geometry, material );
scene.add( object );

camera.position.z = 5;
camera.rotation.z = 180;
camera.rotation.x = 6.125
object.rotation.x = 15;


gsap.registerPlugin(ScrollTrigger);

gsap.to(object.rotation, {
  scrollTrigger: {
  trigger: "#trigger",
  start: "top top",
  end: "bottom top",
  scrub: true,
  toggleActions: "restart pause resume pause"
},
    y: Math.PI,
});

function animate() {
    // object.rotation.x += 0.0050;
    object.rotation.y += 0.0025;
	renderer.render( scene, camera );
}
renderer.setAnimationLoop( animate );
