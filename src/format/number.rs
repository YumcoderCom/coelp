// Copyright 2023-present The Yumcoder Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
//
// Author: yumcoder (omid.jn@gmail.com)
//
const THOUSAND: u64 = 1000;
const MILLION: u64 = THOUSAND * 1000;
const BILLION: u64 = MILLION * 1000;

pub fn human_number(b: u64) -> String {
    match b {
        _ if b > BILLION => format!("{:.0}B", (b as f64) / BILLION as f64),
        _ if b > MILLION => format!("{:.0}M", (b as f64) / MILLION as f64),
        _ if b > THOUSAND => format!("{:.0}K", (b as f64) / THOUSAND as f64),
        _ => format!("{}", b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_number() {
        assert_eq!(human_number(1000), "1000");
        assert_eq!(human_number(1500), "2K");
        assert_eq!(human_number(1000000), "1000K");
        assert_eq!(human_number(1500000), "2M");
        assert_eq!(human_number(1000000000), "1000M");
        assert_eq!(human_number(1500000000), "2B");
        assert_eq!(human_number(12345), "12K");
    }
}
