// Copyright 2023-present The Yumcoder Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
//
// Author: yumcoder (omid.jn@gmail.com)
//
const FALCON_MODEL_TYPE_7B: u32 = 32;
const FALCON_MODEL_TYPE_40B: u32 = 60;
const FALCON_MODEL_TYPE_180B: u32 = 80;

pub fn falcon_model_type(num_layer: u32) -> &'static str {
    match num_layer {
        FALCON_MODEL_TYPE_7B => "7B",
        FALCON_MODEL_TYPE_40B => "40B",
        FALCON_MODEL_TYPE_180B => "180B",
        _ => "unknown",
    }
}
