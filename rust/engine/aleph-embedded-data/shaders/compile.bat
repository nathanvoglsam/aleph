@echo off
dxc /T ps_6_0 -Fo .\compiled\standard\standard.frag.spv -spirv .\source\standard\standard.frag.hlsl -O3 -I .\include -I .\source
dxc /T vs_6_0 -Fo .\compiled\standard\standard.vert.spv -spirv .\source\standard\standard.vert.hlsl -O3 -I .\include -I .\source
dxc /T ps_6_0 -Fo .\compiled\standard_tex\standard_tex.frag.spv -spirv .\source\standard_tex\standard_tex.frag.hlsl -O3 -I .\include -I .\source
dxc /T vs_6_0 -Fo .\compiled\standard_tex\standard_tex.vert.spv -spirv .\source\standard_tex\standard_tex.vert.hlsl -O3 -I .\include -I .\source
dxc /T ps_6_0 -Fo .\compiled\egui\egui.frag.spv -spirv .\source\egui\egui.frag.hlsl -O3 -I .\include -I .\source
dxc /T vs_6_0 -Fo .\compiled\egui\egui.vert.spv -spirv .\source\egui\egui.vert.hlsl -O3 -I .\include -I .\source
dxc /T ps_6_0 -Fo .\compiled\postprocess\tonemapping.frag.spv -spirv .\source\postprocess\tonemapping.frag.hlsl -O3 -I .\include -I .\source
dxc /T vs_6_0 -Fo .\compiled\fullscreen_quad\fullscreen_quad.vert.spv -spirv .\source\fullscreen_quad\fullscreen_quad.vert.hlsl -O3 -I .\include -I .\source
