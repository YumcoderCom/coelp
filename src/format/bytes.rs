// Copyright 2023-present The Yumcoder Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
//
// Author: yumcoder (omid.jn@gmail.com)
//
const BYTE: i64 = 1;
const KILOBYTE: i64 = BYTE * 1000;
const MEGABYTE: i64 = KILOBYTE * 1000;
const GIGABYTE: i64 = MEGABYTE * 1000;

pub fn human_bytes(b: i64) -> String {
    match b {
        _ if b > GIGABYTE => format!("{} GB", b / GIGABYTE),
        _ if b > MEGABYTE => format!("{} MB", b / MEGABYTE),
        _ if b > KILOBYTE => format!("{} KB", b / KILOBYTE),
        _ => format!("{} B", b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_bytes() {
        assert_eq!(human_bytes(100), "100 B");
        assert_eq!(human_bytes(1000), "1000 B");
        assert_eq!(human_bytes(1500), "1 KB");
        assert_eq!(human_bytes(1500000), "1 MB");
        assert_eq!(human_bytes(1500000000), "1 GB");
    }
}
