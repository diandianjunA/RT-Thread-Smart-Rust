use crate::spec::{Target, TargetOptions, base};

pub fn target() -> Target {
    let mut base = base::rtsmart_base::opts();
    base.max_atomic_width = Some(128);
    base.env = "gnu".into();
    base.linker =  Some("aarch64-linux-musleabi-gcc".into());
    
    Target {
        llvm_target: "aarch64-unknown-linux-gnu".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".into(),
        arch: "aarch64".into(),

        options: TargetOptions {
            mcount: "\u{1}_mcount".into(),
            ..base
        },
    }
}
