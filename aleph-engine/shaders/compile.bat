@echo off
dxc /T ps_6_0 -Fo .\compiled\standard\standard.frag.spv -spirv .\source\standard\standard.frag.hlsl -O3 -I .\include
dxc /T vs_6_0 -Fo .\compiled\standard\standard.vert.spv -spirv .\source\standard\standard.vert.hlsl -O3 -I .\include
dxc /T ps_6_0 -Fo .\compiled\standard_tex\standard_tex.frag.spv -spirv .\source\standard_tex\standard_tex.frag.hlsl -O3 -I .\include
dxc /T vs_6_0 -Fo .\compiled\standard_tex\standard_tex.vert.spv -spirv .\source\standard_tex\standard_tex.vert.hlsl -O3 -I .\include
dxc /T ps_6_0 -Fo .\compiled\imgui\imgui.frag.spv -spirv .\source\imgui\imgui.frag.hlsl -O3 -I .\include
dxc /T vs_6_0 -Fo .\compiled\imgui\imgui.vert.spv -spirv .\source\imgui\imgui.vert.hlsl -O3 -I .\include