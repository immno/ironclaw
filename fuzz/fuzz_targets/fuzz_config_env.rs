#![no_main]
use libfuzzer_sys::fuzz_target;

use ironclaw::config::SafetyConfig;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // SafetyConfig fields are parsed from env vars. We cannot safely set
        // env vars in a multi-threaded fuzzer, but we can exercise the types
        // that config parsing produces by constructing SafetyConfig directly
        // and feeding the fuzzed string through the safety layer it creates.

        // Parse the fuzzed input as a potential max_output_length value.
        let max_len: usize = s.parse().unwrap_or(100_000);

        let config = SafetyConfig {
            max_output_length: max_len,
            injection_check_enabled: true,
        };

        // Build a SafetyLayer from the config and exercise it.
        let layer = ironclaw::safety::SafetyLayer::new(&config);

        // Use the fuzzed string as tool output content.
        let _ = layer.sanitize_tool_output("fuzz_tool", s);
        let _ = layer.validate_input(s);
        let _ = layer.check_policy(s);
    }
});
