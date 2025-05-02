use clap::Args;
use procedural::EnvPrefix;

#[derive(Debug, Args, EnvPrefix)]
#[env_prefix = "TEST"]
struct TestConfig {
    #[arg(long, env = "SAMPLE_VAR")]
    sample: String,

    #[arg(long, env = "OTHER_VAR")]
    other: String,
}

#[test]
fn test_env_prefix_transform() {
    unsafe {
        std::env::remove_var("SAMPLE_VAR");
        std::env::remove_var("OTHER_VAR");
        std::env::remove_var("TEST_SAMPLE_VAR");
        std::env::remove_var("TEST_OTHER_VAR");

        // Set up fresh environment
        std::env::set_var("SAMPLE_VAR", "test_value");
        std::env::set_var("OTHER_VAR", "other_value");
    }

    // Transform
    TestConfig::prepend_envs();

    let test_sample = std::env::var("TEST_SAMPLE_VAR").expect("TEST_SAMPLE_VAR should be set");
    let test_other = std::env::var("TEST_OTHER_VAR").expect("TEST_OTHER_VAR should be set");

    assert_eq!(test_sample, "test_value");
    assert_eq!(test_other, "other_value");

    unsafe {
        std::env::remove_var("SAMPLE_VAR");
        std::env::remove_var("OTHER_VAR");
        std::env::remove_var("TEST_SAMPLE_VAR");
        std::env::remove_var("TEST_OTHER_VAR");
    }
}
