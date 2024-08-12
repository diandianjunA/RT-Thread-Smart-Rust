use crate::spec::{Target, TargetOptions};
use crate::spec::base::{rtsmart_base};

// const LINKER_SCRIPT: &str = include_str!("./rtsmart_arm_linker_script.lds");

pub fn target() -> Target {

    let mut base = rtsmart_base::opts();
    base.max_atomic_width = Some(64);
    base.env = "gnu".into();
    base.linker =  Some("arm-linux-musleabi-gcc".into());
    
    Target {
        llvm_target: "armv7-unknown-linux-gnueabi".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),

        options: TargetOptions {
            // Info about features at https://wiki.debian.org/ArmHardFloatPort
            features: "+v7,+vfp3,-d32,+thumb2,-neon".into(),
            cpu: "generic".into(),
            max_atomic_width: Some(64),
            // unsupported_abis: arm_base::unsupported_abis(),
            mcount: "\u{1}__gnu_mcount_nc".into(),
            // linker_flavor: LinkerFlavor::Gnu(crate::spec::Cc::Yes, crate::spec::Lld::Yes),
            // link_script: Some(LINKER_SCRIPT.into()),
            ..base
        },
    }
}
