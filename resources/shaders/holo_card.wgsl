// Holographic card shimmer effect for Bevy 0.18.1
// Adapted from the original Balatro holo.fs (Love2D/GLSL) shader.
// Applies animated rainbow iridescence over the card texture.

#import bevy_ui::ui_vertex_output::UiVertexOutput

struct HoloUniforms {
    holo: vec2<f32>,
    time: f32,
    _pad: f32,
}

@group(1) @binding(0) var<uniform> material: HoloUniforms;
@group(1) @binding(1) var card_texture: texture_2d<f32>;
@group(1) @binding(2) var card_sampler: sampler;

fn hue_to_rgb(s: f32, t: f32, h: f32) -> f32 {
    let hs = (h % 1.0) * 6.0;
    if (hs < 1.0) { return (t - s) * hs + s; }
    if (hs < 3.0) { return t; }
    if (hs < 4.0) { return (t - s) * (4.0 - hs) + s; }
    return s;
}

fn hsl_to_rgb(c: vec4<f32>) -> vec4<f32> {
    if (c.y < 0.0001) {
        return vec4<f32>(vec3<f32>(c.z), c.w);
    }
    let t = select(-c.y * c.z + (c.y + c.z), c.y * c.z + c.z, c.z < 0.5);
    let s = 2.0 * c.z - t;
    return vec4<f32>(
        hue_to_rgb(s, t, c.x + 1.0 / 3.0),
        hue_to_rgb(s, t, c.x),
        hue_to_rgb(s, t, c.x - 1.0 / 3.0),
        c.w,
    );
}

fn rgb_to_hsl(c: vec4<f32>) -> vec4<f32> {
    let lo = min(c.r, min(c.g, c.b));
    let hi = max(c.r, max(c.g, c.b));
    let delta = hi - lo;
    let sum = hi + lo;
    var hsl = vec4<f32>(0.0, 0.0, 0.5 * sum, c.a);
    if (delta < 0.0001) { return hsl; }
    hsl.y = select(delta / (2.0 - sum), delta / sum, hsl.z < 0.5);
    if (hi == c.r) {
        hsl.x = (c.g - c.b) / delta;
    } else if (hi == c.g) {
        hsl.x = (c.b - c.r) / delta + 2.0;
    } else {
        hsl.x = (c.r - c.g) / delta + 4.0;
    }
    hsl.x = (hsl.x / 6.0) % 1.0;
    return hsl;
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    var tex = textureSample(card_texture, card_sampler, uv);

    // Discard fully transparent pixels
    if (tex.a < 0.01) { discard; }

    let hsl = rgb_to_hsl(0.5 * tex + 0.5 * vec4<f32>(0.0, 0.0, 1.0, tex.a));

    let t = material.holo.y * 7.221 + material.time;
    let uv_centered = (uv - 0.5) * 250.0;

    let fp1 = uv_centered + 50.0 * vec2<f32>(sin(-t / 143.634), cos(-t / 99.432));
    let fp2 = uv_centered + 50.0 * vec2<f32>(cos( t / 53.153),  cos( t / 61.453));
    let fp3 = uv_centered + 50.0 * vec2<f32>(sin(-t / 87.532),  sin(-t / 49.000));

    let field = (1.0 + (
        cos(length(fp1) / 19.483) +
        sin(length(fp2) / 33.155) * cos(fp2.y / 15.73) +
        cos(length(fp3) / 27.193) * sin(fp3.x / 21.92)
    )) / 2.0;

    let res = 0.5 + 0.5 * cos(material.holo.x * 2.612 + (field - 0.5) * 3.14);

    let lo = min(tex.r, min(tex.g, tex.b));
    let hi = max(tex.r, max(tex.g, tex.b));
    let delta = 0.2 + 0.3 * (hi - lo) + 0.1 * hi;

    let gs = 0.79;
    let fac = 0.5 * max(max(
        max(0.0, 7.0 * abs(cos(uv.x * gs * 20.0)) - 6.0),
        max(0.0, 7.0 * cos(uv.y * gs * 45.0 + uv.x * gs * 20.0) - 6.0)
    ), max(0.0, 7.0 * cos(uv.y * gs * 45.0 - uv.x * gs * 20.0) - 6.0));

    var hsl_out = hsl;
    hsl_out.x = hsl.x + res + fac;
    hsl_out.y = hsl.y * 1.3;
    hsl_out.z = hsl.z * 0.6 + 0.4;

    let rgb_out = hsl_to_rgb(hsl_out) * vec4<f32>(0.9, 0.8, 1.2, tex.a);
    var result = (1.0 - delta) * tex + delta * rgb_out;
    if (result.a < 0.7) { result.a = result.a / 3.0; }
    return result;
}
