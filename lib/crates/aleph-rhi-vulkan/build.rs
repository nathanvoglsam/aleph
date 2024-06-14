extern crate aleph_target_build as target;

pub fn main() {
    let target_platform = target::build::target_platform();
    if target_platform.is_ios() {
        println!(
            "cargo:rustc-link-search=framework={}",
            "/Users/nvoglsam/VulkanSDK/1.3.283.0/iOS/lib"
        );
        println!(
            "cargo:rustc-link-search=framework={}",
            "/Users/nvoglsam/VulkanSDK/1.3.283.0/iOS/lib/MoltenVK.xcframework/ios-arm64"
        );
    }
}
