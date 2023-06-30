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
    
    let stored_uv = textureSample(texture, our_sampler, uv/dim);


    var mindist = sqrt((uv.x-stored_uv.x)*(uv.x-stored_uv.x)+(uv.y-stored_uv.y)*(uv.y-stored_uv.y));
    var mini = stored_uv.xy;

    
    
    for(var i: i32 = -1; i <= 1; i+=1) {
        for(var j: i32 = -1; j <= 1; j+=1) {
            // if(i!=0 && j!=0) {
                let ddx = f32(i)*settings.intensity;
                let ddy = f32(j)*settings.intensity;
                // let dist = sqrt(ddx*ddx+ddy*ddy);
                let cur_uv = textureSample(texture,our_sampler,(uv+vec2<f32>(ddx,ddy))/dim).xy;
                // if(cur_uv.r == -1.0) {continue;}

                let dist = sqrt((uv.x-cur_uv.x)*(uv.x-cur_uv.x)+(uv.y-cur_uv.y)*(uv.y-cur_uv.y));
                if(dist <= mindist) {
                    mindist = dist;
                    mini = cur_uv;
                }
            // }
        }
    }
    // return cur_uv;
    // return vec4<f32>(mini.x,0.0,0.0,1.0);
    return vec4<f32>(mini,100000.,1.0);
}