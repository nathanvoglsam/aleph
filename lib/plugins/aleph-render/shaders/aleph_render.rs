// Do not edit manually! File is GENERATED!

#[allow(unused)]
pub mod deferred {
    #[allow(unused)]
    pub fn deferred_lighting_cs() -> aleph_shader_db::ShaderName<'static, aleph_shader_db::Compute> {
        unsafe { aleph_shader_db::ShaderName::<aleph_shader_db::Compute>::new("aleph-render/deferred/deferred_lighting.cs") } // Safety guaranteed by code-gen
    }
    #[allow(unused)]
    pub fn main_gbuffer_frag() -> aleph_shader_db::ShaderName<'static, aleph_shader_db::Fragment> {
        unsafe { aleph_shader_db::ShaderName::<aleph_shader_db::Fragment>::new("aleph-render/deferred/main_gbuffer.frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused)]
    pub fn main_gbuffer_vert() -> aleph_shader_db::ShaderName<'static, aleph_shader_db::Vertex> {
        unsafe { aleph_shader_db::ShaderName::<aleph_shader_db::Vertex>::new("aleph-render/deferred/main_gbuffer.vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused)]
pub mod egui {
    #[allow(unused)]
    pub fn egui_frag() -> aleph_shader_db::ShaderName<'static, aleph_shader_db::Fragment> {
        unsafe { aleph_shader_db::ShaderName::<aleph_shader_db::Fragment>::new("aleph-render/egui/egui.frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused)]
    pub fn egui_vert() -> aleph_shader_db::ShaderName<'static, aleph_shader_db::Vertex> {
        unsafe { aleph_shader_db::ShaderName::<aleph_shader_db::Vertex>::new("aleph-render/egui/egui.vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused)]
pub mod fullscreen_quad {
    #[allow(unused)]
    pub fn fullscreen_quad_vert() -> aleph_shader_db::ShaderName<'static, aleph_shader_db::Vertex> {
        unsafe { aleph_shader_db::ShaderName::<aleph_shader_db::Vertex>::new("aleph-render/fullscreen_quad/fullscreen_quad.vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused)]
pub mod postprocess {
}
#[allow(unused)]
pub mod standard {
    #[allow(unused)]
    pub fn standard_frag() -> aleph_shader_db::ShaderName<'static, aleph_shader_db::Fragment> {
        unsafe { aleph_shader_db::ShaderName::<aleph_shader_db::Fragment>::new("aleph-render/standard/standard.frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused)]
    pub fn standard_vert() -> aleph_shader_db::ShaderName<'static, aleph_shader_db::Vertex> {
        unsafe { aleph_shader_db::ShaderName::<aleph_shader_db::Vertex>::new("aleph-render/standard/standard.vert") } // Safety guaranteed by code-gen
    }
}
#[allow(unused)]
pub mod standard_tex {
    #[allow(unused)]
    pub fn standard_tex_frag() -> aleph_shader_db::ShaderName<'static, aleph_shader_db::Fragment> {
        unsafe { aleph_shader_db::ShaderName::<aleph_shader_db::Fragment>::new("aleph-render/standard_tex/standard_tex.frag") } // Safety guaranteed by code-gen
    }
    #[allow(unused)]
    pub fn standard_tex_vert() -> aleph_shader_db::ShaderName<'static, aleph_shader_db::Vertex> {
        unsafe { aleph_shader_db::ShaderName::<aleph_shader_db::Vertex>::new("aleph-render/standard_tex/standard_tex.vert") } // Safety guaranteed by code-gen
    }
}
