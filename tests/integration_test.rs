use assert_cmd::Command as TestingCommand;
use std::process::Command;

/// Just make sure that rusty_wc behaves the same as wc.
#[test]
// Skip if not on OSX
#[cfg(target_os = "macos")]
fn test_wc_compatibility() {
    let wc_compat_test_cases: Vec<Vec<&str>> = vec![
        vec!["-l", "LICENSE", "CONTRIBUTING.md"],
        vec!["-l", "LICENSE"],
        vec!["-w", "LICENSE", "CONTRIBUTING.md"],
        vec!["-w", "LICENSE"],
        vec!["-c", "LICENSE", "CONTRIBUTING.md"],
        vec!["-c", "LICENSE"],
        vec!["LICENSE", "CONTRIBUTING.md"],
        vec!["LICENSE"],
        vec!["-lw", "LICENSE"],
        vec!["-lc", "LICENSE"],
        vec!["-wc", "LICENSE"],
        vec!["-lwc", "LICENSE"],
        vec!["thisfiledoesntexist"],
    ];

    for wc_args in wc_compat_test_cases {
        // First, just run wc and save the output
        let legit_wc_output = Command::new("wc")
            .args(wc_args.clone())
            .output()
            .expect("Failed to run wc");

        let mut cmd = TestingCommand::cargo_bin("rusty-wc").unwrap();
        let rusty_wc_output = cmd.args(wc_args).output().expect("Failed to run rusty-wc");
        if legit_wc_output.status.success() {
            assert_eq!(legit_wc_output, rusty_wc_output);
        } else {
            // On failure, we don't care about exact match. Just similar exit status and error message existence.
            assert_eq!(legit_wc_output.status, rusty_wc_output.status);
            if legit_wc_output.stderr.len() > 0 {
                assert!(rusty_wc_output.stderr.len() > 0);
                // We don't want to assert on the errors, but just for sanity, let's print them.
                // To see this, run `cargo test -- --nocapture`.
                println!(
                    "legit_wc_output.stderr: {}rusty_wc_output.stderr: {}",
                    String::from_utf8_lossy(&legit_wc_output.stderr),
                    String::from_utf8_lossy(&rusty_wc_output.stderr),
                );
            }
        }
    }
}

#[test]
// Skip if not on OSX
#[cfg(target_os = "macos")]
fn test_rusty_wc_frequency_feature() {
    let frequency_license_expec_output: &str =
        "     309 the\n     208 of\n     174 to\n     165 a\n     131 or\n     102 you\n      89 that\n      86 and\n      72 this\n      70 for\n";
    let frequency_license_contributing_expec_output: &str =
        "     318 the\n     208 of\n     182 to\n     168 a\n     131 or\n     104 you\n      93 that\n      88 and\n      72 this\n      70 for\n";

    let rusty_wc_frequency_expec_output_strings: [&str; 2] = [
        &frequency_license_expec_output,
        &frequency_license_contributing_expec_output,
    ];

    let rusty_wc_frequency_test_cases: Vec<Vec<&str>> = vec![
        vec!["-f", "LICENSE"],
        vec!["-f", "LICENSE", "CONTRIBUTING.md"],
    ];

    for (i, rusty_wc_args) in rusty_wc_frequency_test_cases.iter().enumerate() {
        let mut cmd = TestingCommand::cargo_bin("rusty-wc").unwrap();
        let rusty_wc_output = cmd
            .args(rusty_wc_args)
            .output()
            .expect("Failed to run rusty-wc");

        if rusty_wc_output.status.success() {
            let rusty_wc_output_string: String =
                String::from_utf8(rusty_wc_output.stdout).expect("Output is invalid");
            assert_eq!(rusty_wc_output_string, rusty_wc_frequency_expec_output_strings[i]);
        }
    }
}
