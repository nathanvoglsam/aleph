//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

/*
 * The standard vertex input layout that a fullscreen quad pass uses
 */
struct FSQuadVertexLayout {
    [[vk::location(0)]] float2 Pos   : SV_POSITION;
};
