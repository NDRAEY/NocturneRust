use crate::spec::{base, PanicStrategy, Target, TargetMetadata};

pub(crate) fn target() -> Target {
    let mut base = base::nocturne::opts();
    base.cpu = "i686".into();
    base.disable_redzone = true;
    base.panic_strategy = PanicStrategy::Abort;
    base.link_script = Some(include_str!("./nocturne-linker.ld").into());
    base.features = "-mmx,-sse,+soft-float".into();

    Target {
        llvm_target: "i686-unknown-none".into(),
        pointer_width: 32,
        data_layout:
            "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-i128:128-f64:32:64-f80:32-n8:16:32-S128".into(),
        arch: "x86".into(),
        options: base,
        metadata: TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
    }
}
