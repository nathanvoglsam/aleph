// Do not edit manually! File is GENERATED!

#[allow(unused)]
use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

#[allow(unused, non_snake_case)]
pub mod composite_planes {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused, non_snake_case)]
    pub const fn frag() -> ShaderName<'static, Fragment> {
        unsafe { ShaderName::<Fragment>::new("aleph-renderer/composite_planes/frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused, non_snake_case)]
    pub const fn vert() -> ShaderName<'static, Vertex> {
        unsafe { ShaderName::<Vertex>::new("aleph-renderer/composite_planes/vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused, non_snake_case)]
pub mod deferred {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused, non_snake_case)]
    pub const fn deferred_lighting_cs() -> ShaderName<'static, Compute> {
        unsafe { ShaderName::<Compute>::new("aleph-renderer/deferred/deferred_lighting.cs") } // Safety guaranteed by code-gen
    }
    #[allow(unused, non_snake_case)]
    pub const fn main_gbuffer_frag() -> ShaderName<'static, Fragment> {
        unsafe { ShaderName::<Fragment>::new("aleph-renderer/deferred/main_gbuffer.frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused, non_snake_case)]
    pub const fn main_gbuffer_vert() -> ShaderName<'static, Vertex> {
        unsafe { ShaderName::<Vertex>::new("aleph-renderer/deferred/main_gbuffer.vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused, non_snake_case)]
pub mod fullscreen_quad {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused, non_snake_case)]
    pub const fn fullscreen_quad_vert() -> ShaderName<'static, Vertex> {
        unsafe { ShaderName::<Vertex>::new("aleph-renderer/fullscreen_quad/fullscreen_quad.vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused, non_snake_case)]
pub mod fxaa {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused, non_snake_case)]
    pub const fn frag() -> ShaderName<'static, Fragment> {
        unsafe { ShaderName::<Fragment>::new("aleph-renderer/fxaa/frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused, non_snake_case)]
    pub const fn vert() -> ShaderName<'static, Vertex> {
        unsafe { ShaderName::<Vertex>::new("aleph-renderer/fxaa/vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused, non_snake_case)]
pub mod postprocess {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused, non_snake_case)]
    pub const fn tonemapping_cs() -> ShaderName<'static, Compute> {
        unsafe { ShaderName::<Compute>::new("aleph-renderer/postprocess/tonemapping.cs") } // Safety guaranteed by code-gen
    }
}
