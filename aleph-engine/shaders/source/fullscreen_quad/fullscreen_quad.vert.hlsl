//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include "fullscreen_quad.inc.hlsl"

void main(in FSQuadVertexLayout input, out float4 Pos : SV_POSITION) {
	Pos = float4(input.Pos, 1);
}
