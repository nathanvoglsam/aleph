@echo off
dxc /T ps_6_0 -Fo .\compiled\standard\standard.frag.dxil .\source\standard\standard.frag.hlsl -O3 -I .\include -I .\source -Wno-ignored-attributes
dxc /T vs_6_0 -Fo .\compiled\standard\standard.vert.dxil .\source\standard\standard.vert.hlsl -O3 -I .\include -I .\source -Wno-ignored-attributes
dxc /T ps_6_0 -Fo .\compiled\standard_tex\standard_tex.frag.dxil .\source\standard_tex\standard_tex.frag.hlsl -O3 -I .\include -I .\source -Wno-ignored-attributes
dxc /T vs_6_0 -Fo .\compiled\standard_tex\standard_tex.vert.dxil .\source\standard_tex\standard_tex.vert.hlsl -O3 -I .\include -I .\source -Wno-ignored-attributes
dxc /T ps_6_0 -Fo .\compiled\egui\egui.frag.dxil .\source\egui\egui.frag.hlsl -O3 -I .\include -I .\source -Wno-ignored-attributes
dxc /T vs_6_0 -Fo .\compiled\egui\egui.vert.dxil .\source\egui\egui.vert.hlsl -O3 -I .\include -I .\source -Wno-ignored-attributes
dxc /T ps_6_0 -Fo .\compiled\postprocess\tonemapping.frag.dxil .\source\postprocess\tonemapping.frag.hlsl -O3 -I .\include -I .\source -Wno-ignored-attributes
dxc /T vs_6_0 -Fo .\compiled\fullscreen_quad\fullscreen_quad.vert.dxil .\source\fullscreen_quad\fullscreen_quad.vert.hlsl -O3 -I .\include -I .\source -Wno-ignored-attributes
