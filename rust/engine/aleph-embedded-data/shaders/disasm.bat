@echo off
spirv-dis .\compiled\standard\standard.frag.spv > .\disasm\standard\standard.frag.spirv
spirv-dis .\compiled\standard\standard.vert.spv > .\disasm\standard\standard.vert.spirv
spirv-dis .\compiled\standard_tex\standard_tex.frag.spv > .\disasm\standard_tex\standard_tex.frag.spirv
spirv-dis .\compiled\standard_tex\standard_tex.vert.spv > .\disasm\standard_tex\standard_tex.vert.spirv
spirv-dis .\compiled\imgui\imgui.frag.spv > .\disasm\imgui\imgui.frag.spirv
spirv-dis .\compiled\imgui\imgui.vert.spv > .\disasm\imgui\imgui.vert.spirv
spirv-dis .\compiled\egui\egui.frag.spv > .\disasm\egui\egui.frag.spirv
spirv-dis .\compiled\egui\egui.vert.spv > .\disasm\egui\egui.vert.spirv
spirv-dis .\compiled\postprocess\tonemapping.frag.spv > .\disasm\postprocess\tonemapping.frag.spirv
spirv-dis .\compiled\fullscreen_quad\fullscreen_quad.vert.spv > .\disasm\fullscreen_quad\fullscreen_quad.vert.spirv
