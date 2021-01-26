# Project Back Under Control

- Need to audit that state of all the current crates and decide what needs to be done with each one.
  There's a lot of old code that hasn't been touched in over 6 months left in some state of
  completion or another. Need to figure out what is finished, what isn't, what is worth finishing
  and what should just be removed.
  
- Need a plan moving forward to get to something that actually works.

# Crate Statuses

## Done and stable, doesn't need to be touched

- `aleph-compile`
    - Very straight forward interface, won't need to change until porting to more platforms
- `aleph-target`
    - Same as above
- `aleph-target-build`
    - Same as above
- `aleph-macros`
    - Just an artifact of code deduplication where general purpose code will be re-used in multiple
      places, but isn't enough to justify their own crates
- `aleph-log`
    - Wrapper for `log` to ensure everything is on the same version
- `aleph-logger`
    - Platform abstraction for swapping between different logger implementations in different
      platforms. Will be very stable, not much needs to change
- `aleph-platform`
    - This is some of the better designed code. A thread safe, easy to use wrapper over the
      underlying SDL2 platform layer. This should be the only point that touches SDL2 directly as I
      want the flexibility to replace it in the future
- `aleph-cpu-info`
    - Wrapper for another rust crate that simplifies the API

## Unfinished, but should be completed

- `aleph-rust-codegen`
    - Keep on back burner until a strong need for it exists
- `aleph-rust-parser`
    - Same as above
- `aleph-ktx`
    - Need to verify it matches the latest version of the spec
    - Need to complete file validation
- `aleph-vk-format`
    - Need to make sure this contains all vulkan formats

## Unfinished and should abandon

- `aleph-platform-imgui`
    - Going to dump imgui for egui

## Needs redesign

- `aleph-app-info`
    - Poorly thought out interface, may merge with `aleph-settings` into a registry like object
- `aleph-settings`
    - Poorly thought out interface
- `aleph-embedded-data`
    - Should dump this and move to 100% data driven, nothing built in
    
## Needs Evaluation

- `aleph-engine`
    - The core driver for the engine. Haven't looked at this in ages, should re-evaluate
- `aleph-render`
- `aleph-render-graph`
- `aleph-vulkan`
- `aleph-vulkan-alloc`
- `aleph-vulkan-alloc-sys`
- `aleph-vulkan-core`
