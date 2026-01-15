//! Fuzzing target for HTML parsing.
//!
//! This target exercises the HTML parser with random input to find edge cases
//! and potential crashes.

#![no_main]

use libfuzzer_sys::fuzz_target;
use scrape_core::Soup;

fuzz_target!(|data: &[u8]| {
    // Try to parse as UTF-8 string
    if let Ok(html) = std::str::from_utf8(data) {
        // Parse with default config
        let _ = Soup::parse(html);

        // Parse with strict mode
        let config = scrape_core::SoupConfig::builder().strict_mode(true).build();
        let _ = Soup::parse_with_config(html, config);

        // Parse with limited depth
        let config = scrape_core::SoupConfig::builder().max_depth(32).build();
        let _ = Soup::parse_with_config(html, config);
    }
});
