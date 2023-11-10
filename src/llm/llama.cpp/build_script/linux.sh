#!/bin/bash
#
# Copyright 2023-present The Yumcoder Authors. All rights reserved.
# Use of this source code is governed by a BSD-style
# license that can be found in the LICENSE file.
# 
# Author: yumcoder (omid.jn@gmail.com)
# 

echo "Initiate a build for linux"

git submodule init

git submodule update --force ggml
git -C ggml apply ../patches/0001-add-detokenize-endpoint.patch
git -C ggml apply ../patches/0002-34B-model-support.patch
git -C ggml apply ../patches/0005-ggml-support-CUDA-s-half-type-for-aarch64-1455-2670.patch
git -C ggml apply ../patches/0001-copy-cuda-runtime-libraries.patch
cmake -S ggml -B ggml/build/cpu -DLLAMA_K_QUANTS=on
cmake --build ggml/build/cpu --target server --config Release
mv ggml/build/cpu/bin/server ggml/build/cpu/bin/coelp-runner

git submodule update --force gguf
git -C gguf apply ../patches/0001-copy-cuda-runtime-libraries.patch
git -C gguf apply ../patches/0001-update-default-log-target.patch
cmake -S gguf -B gguf/build/cpu -DLLAMA_K_QUANTS=on -DLLAMA_NATIVE=off -DLLAMA_AVX=on -DLLAMA_AVX2=off -DLLAMA_AVX512=off -DLLAMA_FMA=off -DLLAMA_F16C=off
cmake --build gguf/build/cpu --target server --config Release
mv gguf/build/cpu/bin/server gguf/build/cpu/bin/coelp-runner

cmake -S ggml -B ggml/build/cuda -DLLAMA_CUBLAS=on -DLLAMA_ACCELERATE=on -DLLAMA_K_QUANTS=on
cmake --build ggml/build/cuda --target server --config Release
mv ggml/build/cuda/bin/server ggml/build/cuda/bin/coelp-runner
cmake -S gguf -B gguf/build/cuda -DLLAMA_CUBLAS=on -DLLAMA_ACCELERATE=on -DLLAMA_K_QUANTS=on -DLLAMA_NATIVE=off -DLLAMA_AVX=on -DLLAMA_AVX2=off -DLLAMA_AVX512=off -DLLAMA_FMA=off -DLLAMA_F16C=off
cmake --build gguf/build/cuda --target server --config Release
mv gguf/build/cuda/bin/server gguf/build/cuda/bin/coelp-runner