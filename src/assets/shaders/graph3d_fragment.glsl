precision mediump float;

varying lowp vec4 vColour;

void main() {
    gl_FragColor = vec4(0.5, 0.5, 0.8, 1.0);//vec4(vColour.r, vColour.g, vColour.b, vColour.a);
}