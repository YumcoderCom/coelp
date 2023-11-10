# Copyright 2023-present The Yumcoder Authors. All rights reserved.
# Use of this source code is governed by a BSD-style
# license that can be found in the LICENSE file.
# 
# Author: yumcoder (omid.jn@gmail.com)
# 
echo "Initiate a build for windows"

git submodule init

git submodule update --force ggml
git -C ggml apply ../patches/0001-add-detokenize-endpoint.patch
git -C ggml apply ../patches/0002-34B-model-support.patch
cmake -S ggml -B ggml/build/cpu -DLLAMA_K_QUANTS=on
cmake --build ggml/build/cpu --target server --config Release
cmd /c move ggml\build\cpu\bin\Release\server.exe ggml\build\cpu\bin\Release\coelp-runner.exe

git submodule update --force gguf
git -C gguf apply ../patches/0001-update-default-log-target.patch
cmake -S gguf -B gguf/build/cpu -DLLAMA_K_QUANTS=on -DLLAMA_NATIVE=off -DLLAMA_AVX=on -DLLAMA_AVX2=off -DLLAMA_AVX512=off -DLLAMA_FMA=off -DLLAMA_F16C=off
cmake --build gguf/build/cpu --target server --config Release
cmd /c move gguf\build\cpu\bin\Release\server.exe gguf\build\cpu\bin\Release\coelp-runner.exe