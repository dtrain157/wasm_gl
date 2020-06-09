import * as wasm from "wasm_gl";

const canvas = document.getElementById("wasmCanvas");
const gl = canvas.getContext("webgl", { antialias: true });
const wasmWebGlClient = new wasm.WebGlClient();

if (!gl) {
  alert("Failed to initialize WebGL");
} else {
  const FPS_THROTTLE = 1000.0 / 60.0;
  const initialTime = Date.now();
  var lastDrawTime = -1;

  function render() {
    window.requestAnimationFrame(render);
    const currentTime = Date.now();

    if (currentTime >= lastDrawTime + FPS_THROTTLE) {
      lastDrawTime = currentTime;

      checkWindowDimentions();

      var elapsedTime = currentTime - initialTime;
      wasmWebGlClient.update(elapsedTime, window.innerHeight, window.innerWidth);
      wasmWebGlClient.render();
    }
  }

  function checkWindowDimentions() {
    if (window.innerHeight != canvas.height || window.innerWidth != canvas.width) {
      canvas.height = window.innerHeight;
      canvas.style.height = window.innerHeight;
      canvas.width = window.innerWidth;
      canvas.style.width = window.innerWidth;

      gl.viewport(0, 0, window.innerWidth, window.innerHeight);
    }
  }

  render();
}
