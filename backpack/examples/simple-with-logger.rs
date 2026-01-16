use log_rs::logging::{LogFormat, ModernLogger, ModernBackend, Printer, Verbosity, L, log, log::*, set_logger};
use policy_rs::policy::{Policy, ReasonCode, Request, Rule, Target};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger with verbosity
    let logger = Printer::new(ModernLogger, ModernBackend, LogFormat::Text, Verbosity::Normal);
    set_logger(logger);

    L.intro("Evaluating policy");
    L.step("Building policy with allow rule");

    let policy = Policy::builder()
        .rule(Rule::allow(Target::any(), ReasonCode(1)))
        .build()?;

    log::ok("Policy built successfully");
    log::step("Evaluating request for alice/read/doc");

    let decision = policy.evaluate(&Request::new("alice", "read", "doc"))?;

    if decision.is_allow() {
        ok("Access granted");
    } else {
        warn("Access denied");
    }

    assert!(decision.is_allow());
    outro("Policy evaluation complete");

    Ok(())
}
