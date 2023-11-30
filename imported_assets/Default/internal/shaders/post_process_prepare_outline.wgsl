#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils


@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

fn is_body(p: vec2<f32>) -> bool {
    return textureSample(texture, our_sampler, p).r > 0.0;
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = position.xy;

    let dim = vec2<f32>(textureDimensions(texture));
    
    let c = uv;
   
    // var mindist = 100000.;
    // var mini: vec2<f32> = c * f32(is_body(c));
    // var mini: vec2<f32> = c;

    
    // for(var i = -1; i <= 1; i+=1) {
    //     for(var j = -1; j <= 1; j+=1) {
    //         if(i!=0 && j!=0) {
    //             let ddx = dx*f32(i)*8.;
    //             let ddy = dy*f32(j)*8.;
    //             let dist = sqrt(ddx*ddx+ddy*ddy);
    //             if(dist < mindist && is_body(vec2<f32>(uv.x+ddx,uv.y+ddy)) 
    //                         && (!is_body(vec2<f32>(uv.x+ddx+dx,uv.y+ddy))
    //                         || !is_body(vec2<f32>(uv.x+ddx,uv.y+ddy+dy))
    //                         || !is_body(vec2<f32>(uv.x+ddx-dx,uv.y+ddy))
    //                         || !is_body(vec2<f32>(uv.x+ddx,uv.y+ddy-dy)))
    //             ) {
    //                 mindist = dist;
    //                 mini = vec2<f32>(uv.x+ddx,uv.y+ddy);
    //             }
    //         }
    //     }
    // }


    if (
        !is_body(c/dim) 
        && (
           is_body(vec2<f32>(uv.x+( 1.),uv.y      )/dim)
        || is_body(vec2<f32>(uv.x      ,uv.y+( 1.))/dim)
        || is_body(vec2<f32>(uv.x+(-1.),uv.y      )/dim)
        || is_body(vec2<f32>(uv.x      ,uv.y+(-1.))/dim)
        || is_body(vec2<f32>(uv.x+(-1.),uv.y+(-1.))/dim)
        || is_body(vec2<f32>(uv.x+( 1.),uv.y+(-1.))/dim)
        || is_body(vec2<f32>(uv.x+(-1.),uv.y+( 1.))/dim)
        || is_body(vec2<f32>(uv.x+( 1.),uv.y+( 1.))/dim)
    )) {
        return vec4<f32>(c,0.0,1.0);
    } else if (is_body(c)) {
        return vec4<f32>(-1.0,-1.0,1.0,1.0);
    } else {
        return vec4<f32>(-1.0,-1.0,-1.0,1.0);
    }
        // return vec4<f32>(textureSample(texture,our_sampler,c/dim).xy,1.0,1.0);
    
}