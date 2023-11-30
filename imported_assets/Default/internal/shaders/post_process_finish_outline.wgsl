#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils


@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

struct PostProcessSettings {
    intensity: f32,
}
@group(1) @binding(2)
var<uniform> settings: PostProcessSettings;

fn tsw(t_diffuse: texture_2d<f32>, s_diffuse: sampler, uv: vec2<f32>) -> vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, uv);
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = position.xy;


    let dim = vec2<f32>(textureDimensions(texture));
    // let dx = f32(dim.x);
    // let dy = f32(dim.y);
    
    
    let stored_uv = textureSample(texture, our_sampler, uv/dim);
    // return vec4<f32>((stored_uv.xy-uv.xy)*(stored_uv.xy-uv.xy)/80.,0.0,1.0);


    var mindist = ((uv.x-stored_uv.x)*(uv.x-stored_uv.x)+(uv.y-stored_uv.y)*(uv.y-stored_uv.y));
    let n = f32(mindist < 12.0);
    return vec4<f32>(n,0.0,0.0,n);
    //     // ((uv.x-stored_uv.x)*(uv.x-stored_uv.x)+(uv.y-stored_uv.y)*(uv.y-stored_uv.y));
    
    // let a = (settings.intensity+100.0-mindist);

    // // if(mindist<settings.intensity*100.) {
    // //     return vec4<f32>(1.0,1.0,1.0,1.0);
    // // } else if(mindist>=settings.intensity*settings.intensity && mindist<=(settings.intensity+1.0)*(settings.intensity+1.0)) {
    //     return vec4<f32>(a,a,a,1.0);
    // // } else {
    // //     return vec4<f32>(0.0,0.0,0.0,1.0);
    // // }

    // // return vec4<f32>(uv.x/2000.,stored_uv.x/100.,stored_uv.z,1.0);
    
}