// Foil card shimmer effect for Bevy 0.18.1
// Adapted from the original Balatro foil.fs (Love2D/GLSL) shader.
// Applies a blue-shifted specular shimmer to card edges and highlights.

#import bevy_ui::ui_vertex_output::UiVertexOutput

struct FoilUniforms {
    foil: vec2<f32>,
    time: f32,
    _pad: f32,
}

@group(1) @binding(0) var<uniform> material: FoilUniforms;
@group(1) @binding(1) var card_texture: texture_2d<f32>;
@group(1) @binding(2) var card_sampler: sampler;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    var tex = textureSample(card_texture, card_sampler, uv);

    if (tex.a < 0.01) { discard; }

    let adjusted = uv - vec2<f32>(0.5, 0.5);

    let lo = min(tex.r, min(tex.g, tex.b));
    let hi = max(tex.r, max(tex.g, tex.b));
    let delta = min(hi, max(0.5, 1.0 - lo));

    // Primary shimmer lobe
    let fac = max(min(
        2.0 * sin((length(90.0 * adjusted) + material.foil.x * 2.0)
            + 3.0 * (1.0 + 0.8 * cos(length(113.112 * adjusted) - material.foil.x * 3.121))) - 1.0
        - max(5.0 - length(90.0 * adjusted), 0.0),
        1.0
    ), 0.0);

    // Rotation-based secondary lobe
    let rotater = vec2<f32>(cos(material.foil.x * 0.1221), sin(material.foil.x * 0.3512));
    let angle = dot(rotater, adjusted) / (length(rotater) * max(length(adjusted), 0.0001));
    let fac2 = max(min(
        5.0 * cos(material.foil.y * 0.3 + angle * 3.14 * (2.2 + 0.9 * sin(material.foil.x * 1.65 + 0.2 * material.foil.y)))
        - 4.0 - max(2.0 - length(20.0 * adjusted), 0.0),
        1.0
    ), 0.0);

    let fac3 = 0.3 * max(min(
        2.0 * sin(material.foil.x * 5.0 + uv.x * 3.0 + 3.0 * (1.0 + 0.5 * cos(material.foil.x * 7.0))) - 1.0,
        1.0
    ), -1.0);
    let fac4 = 0.3 * max(min(
        2.0 * sin(material.foil.x * 6.66 + uv.y * 3.8 + 3.0 * (1.0 + 0.5 * cos(material.foil.x * 3.414))) - 1.0,
        1.0
    ), -1.0);

    let maxfac = max(
        max(fac, max(fac2, max(fac3, max(fac4, 0.0)))) + 2.2 * (fac + fac2 + fac3 + fac4),
        0.0
    );

    // Apply blue-shifted foil highlight
    var result = tex;
    result.r = tex.r - delta + delta * maxfac * 0.3;
    result.g = tex.g - delta + delta * maxfac * 0.3;
    result.b = tex.b + delta * maxfac * 1.9;
    result.a = min(tex.a, 0.3 * tex.a + 0.9 * min(0.5, maxfac * 0.1));
    return result;
}
