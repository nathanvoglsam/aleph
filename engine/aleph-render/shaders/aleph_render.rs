// Do not edit manually! File is GENERATED!

#[allow(unused)]
use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

#[allow(unused, non_snake_case)]
pub mod egui {
    #[allow(unused)]
    use aleph_shader_db::{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex };

    #[allow(unused, non_snake_case)]
    pub const fn egui_frag() -> ShaderName<'static, Fragment> {
        unsafe { ShaderName::<Fragment>::new("aleph-render/egui/egui.frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused, non_snake_case)]
    pub const fn egui_vert() -> ShaderName<'static, Vertex> {
        unsafe { ShaderName::<Vertex>::new("aleph-render/egui/egui.vert") } // Safety guaranteed by code-gen
    }
}
