attribute vec4 aPosition;

uniform mat4 uViewProjection;
uniform mat4 uModel;
varying lowp vec4 vColour;

void main() {
    gl_Position = uModel * uViewProjection * vec4(aPosition.x, 0.0, aPosition.z, 1.0);

    vColour = vec4(0.5, 0.5, 0.8, 1.0);
}