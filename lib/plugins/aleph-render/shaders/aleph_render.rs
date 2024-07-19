// Do not edit manually! File is GENERATED!

#[allow(unused)]
use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

#[allow(unused)]
pub mod deferred {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused)]
    pub fn deferred_lighting_cs() -> ShaderName<'static, Compute> {
        unsafe { ShaderName::<Compute>::new("aleph-render/deferred/deferred_lighting.cs") } // Safety guaranteed by code-gen
    }
    #[allow(unused)]
    pub fn main_gbuffer_frag() -> ShaderName<'static, Fragment> {
        unsafe { ShaderName::<Fragment>::new("aleph-render/deferred/main_gbuffer.frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused)]
    pub fn main_gbuffer_vert() -> ShaderName<'static, Vertex> {
        unsafe { ShaderName::<Vertex>::new("aleph-render/deferred/main_gbuffer.vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused)]
pub mod egui {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused)]
    pub fn egui_frag() -> ShaderName<'static, Fragment> {
        unsafe { ShaderName::<Fragment>::new("aleph-render/egui/egui.frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused)]
    pub fn egui_vert() -> ShaderName<'static, Vertex> {
        unsafe { ShaderName::<Vertex>::new("aleph-render/egui/egui.vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused)]
pub fn fullscreen_tri_copy_frag() -> ShaderName<'static, Fragment> {
    unsafe { ShaderName::<Fragment>::new("aleph-render/fullscreen-tri-copy.frag") } // Safety guaranteed by code-gen
}
#[allow(unused)]
pub fn fullscreen_tri_vert() -> ShaderName<'static, Vertex> {
    unsafe { ShaderName::<Vertex>::new("aleph-render/fullscreen-tri.vert") } // Safety guaranteed by code-gen
}
#[allow(unused)]
pub mod fullscreen_quad {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused)]
    pub fn fullscreen_quad_vert() -> ShaderName<'static, Vertex> {
        unsafe { ShaderName::<Vertex>::new("aleph-render/fullscreen_quad/fullscreen_quad.vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused)]
pub mod postprocess {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused)]
    pub fn tonemapping_cs() -> ShaderName<'static, Compute> {
        unsafe { ShaderName::<Compute>::new("aleph-render/postprocess/tonemapping.cs") } // Safety guaranteed by code-gen
    }
}
#[allow(unused)]
pub mod standard {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused)]
    pub fn standard_frag() -> ShaderName<'static, Fragment> {
        unsafe { ShaderName::<Fragment>::new("aleph-render/standard/standard.frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused)]
    pub fn standard_vert() -> ShaderName<'static, Vertex> {
        unsafe { ShaderName::<Vertex>::new("aleph-render/standard/standard.vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused)]
pub mod standard_tex {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused)]
    pub fn standard_tex_frag() -> ShaderName<'static, Fragment> {
        unsafe { ShaderName::<Fragment>::new("aleph-render/standard_tex/standard_tex.frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused)]
    pub fn standard_tex_vert() -> ShaderName<'static, Vertex> {
        unsafe { ShaderName::<Vertex>::new("aleph-render/standard_tex/standard_tex.vert") } // Safety guaranteed by code-gen
    }
}
