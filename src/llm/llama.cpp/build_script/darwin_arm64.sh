#!/bin/bash

echo "start build llama.cpp for darwin_arm64"

git submodule init

git submodule update --force ggml
git -C ggml apply ../patches/0001-add-detokenize-endpoint.patch
git -C ggml apply ../patches/0002-34B-model-support.patch
git -C ggml apply ../patches/0003-metal-fix-synchronization-in-new-matrix-multiplicati.patch
git -C ggml apply ../patches/0004-metal-add-missing-barriers-for-mul-mat-2699.patch
# cmake -S ggml -B ggml/build/metal -DLLAMA_METAL=on -DLLAMA_ACCELERATE=on -DLLAMA_K_QUANTS=on -DCMAKE_SYSTEM_PROCESSOR=arm64 -DCMAKE_OSX_ARCHITECTURES=arm64 -DCMAKE_OSX_DEPLOYMENT_TARGET=11.0
# cmake --build ggml/build/metal --target server --config Release
# mv ggml/build/metal/bin/server ggml/build/metal/bin/coelp-runner

# git submodule update --force gguf
# git -C gguf apply ../patches/0001-update-default-log-target.patch
# git -C gguf apply ../patches/0001-metal-handle-ggml_scale-for-n-4-0-close-3754.patch
# cmake -S gguf -B gguf/build/metal -DLLAMA_METAL=on -DLLAMA_ACCELERATE=on -DLLAMA_K_QUANTS=on -DCMAKE_SYSTEM_PROCESSOR=arm64 -DCMAKE_OSX_ARCHITECTURES=arm64 -DCMAKE_OSX_DEPLOYMENT_TARGET=11.0
# cmake --build gguf/build/metal --target server --config Release
# mv gguf/build/metal/bin/server gguf/build/metal/bin/coelp-runner
