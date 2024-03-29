#!/bin/bash
#
# Copyright 2023-present The Yumcoder Authors. All rights reserved.
# Use of this source code is governed by a BSD-style
# license that can be found in the LICENSE file.
# 
# Author: yumcoder (omid.jn@gmail.com)
# 
echo "Initiate a build for darwin_amd64"

git submodule init -C ..

git submodule update --force ../ggml
git -C ../ggml apply ../patches/0001-add-detokenize-endpoint.patch
git -C ../ggml apply ../patches/0002-34B-model-support.patch
git -C ../ggml apply ../patches/0003-metal-fix-synchronization-in-new-matrix-multiplicati.patch
git -C ../ggml apply ../patches/0004-metal-add-missing-barriers-for-mul-mat-2699.patch
cmake -S ggml -B ../ggml/build/cpu -DLLAMA_ACCELERATE=on -DLLAMA_K_QUANTS=on -DCMAKE_SYSTEM_PROCESSOR=x86_64 -DCMAKE_OSX_ARCHITECTURES=x86_64 -DCMAKE_OSX_DEPLOYMENT_TARGET=11.0
cmake --build ../ggml/build/cpu --target server --config Release
mv ../ggml/build/cpu/bin/server ../ggml/build/cpu/bin/coelp-runner

git submodule update --force gguf -C ..
git -C gguf apply ../patches/0001-update-default-log-target.patch
git -C gguf apply ~../patches/0001-metal-handle-ggml_scale-for-n-4-0-close-3754.patch
cmake -S gguf -B ../gguf/build/cpu -DLLAMA_ACCELERATE=on -DLLAMA_K_QUANTS=on -DCMAKE_SYSTEM_PROCESSOR=x86_64 -DCMAKE_OSX_ARCHITECTURES=x86_64 -DCMAKE_OSX_DEPLOYMENT_TARGET=11.0
cmake --build ../gguf/build/cpu --target server --config Release
mv ../gguf/build/cpu/bin/server ../gguf/build/cpu/bin/coelp-runner