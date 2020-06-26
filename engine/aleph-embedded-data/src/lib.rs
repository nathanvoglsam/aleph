//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

pub extern crate gltf;

extern crate aleph_macros as macros;
extern crate once_cell;

pub mod fonts;
pub mod shaders;

mod cube_mesh;
mod fullscreen_quad;
mod sphere_mesh;
mod utils;

pub use cube_mesh::CubeMesh;
pub use fullscreen_quad::FullscreenQuad;
pub use sphere_mesh::SphereMesh;
