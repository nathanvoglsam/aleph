//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

extern crate aleph_target_build as target;

fn main() {
    target::build::target_architecture().print_target_cargo_cfg();
    target::build::target_platform().print_target_cargo_cfg();
    target::build::target_build_type().print_target_cargo_cfg();

    target::build::host_architecture().print_host_cargo_cfg();
    target::build::host_platform().print_host_cargo_cfg();
    target::build::host_build_type().print_host_cargo_cfg();
}
