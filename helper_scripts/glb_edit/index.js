const { Document, NodeIO } = require("@gltf-transform/core");
const { KHRONOS_EXTENSIONS } = require("@gltf-transform/extensions");
const f = require("@gltf-transform/functions");
const sharp = require("sharp");

const io = new NodeIO().registerExtensions(KHRONOS_EXTENSIONS);
const document = await io.read("test.glb");

await document.transform(
    f.textureCompress({
        encoder: sharp,
        
    })
);