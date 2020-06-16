attribute vec4 aPosition;
attribute float aY;
attribute vec3 aVertexNormal;

uniform mat4 uNormalsRotation;
uniform mat4 uViewProjection;
uniform mat4 uModel;

varying lowp vec4 vColour;

void main() {
    gl_Position = uViewProjection * uModel * vec4(aPosition.x, aY, aPosition.z, 1.0);

    vec3 ambientLightColour = vec3(0.5, 0.5, 0.5);
    vec3 directionalLightColour = vec3(1.0, 1.0, 1.0);
    vec3 directionalVector = normalize(vec3(-0.8, 0.8, 0.75));

    vec4 transformedNormal = uNormalsRotation * vec4(aVertexNormal, 1.0);
    float directional = max(dot(transformedNormal.xyz, directionalVector), 0.0);
    vec3 vLighting = ambientLightColour + (directionalLightColour * directional);
    
    vec3 baseColour = vec3(0.2, 0.3, 0.8);


    vColour = vec4(baseColour * vLighting, 1.0);

    
}