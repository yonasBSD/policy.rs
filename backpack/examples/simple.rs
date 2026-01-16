use policy_rs::policy::{Policy, ReasonCode, Request, Rule, Target};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let policy = Policy::builder()
        .rule(Rule::allow(Target::any(), ReasonCode(1)))
        .build()?;

    let decision = policy.evaluate(&Request::new("alice", "read", "doc"))?;
    assert!(decision.is_allow());

    Ok(())
}
