use crate::spec::{LinkArgs, LinkerFlavor, TargetOptions, RelroLevel, cvs, PanicStrategy};

// const LINKER_SCRIPT: &str = include_str!("./rtsmart_arm_linker_script.lds");

pub fn opts() -> TargetOptions {
    let mut args = LinkArgs::new();
    args.insert(
        LinkerFlavor::Gnu(crate::spec::Cc::Yes, crate::spec::Lld::No),
        vec![
            // We want to be able to strip as much executable code as possible
            // from the linker command line, and this flag indicates to the
            // linker that it can avoid linking in dynamic libraries that don't
            // actually satisfy any symbols up to that point (as with many other
            // resolutions the linker does). This option only applies to all
            // following libraries so we're sure to pass it as one of the first
            // arguments.
            // "-Wl,--as-needed".to_string(),
            // Always enable NX protection when it is available
            // "-Wl,-z,noexecstack".to_string(),
        ],
    );

    TargetOptions {
        // os: "linux".to_string(),
        // env: "gnu".to_string(),
        // vendor: "unknown".to_string(),
        // linker: Some("arm-linux-musleabi-g++".to_string()),
        // exe_suffix: ".elf".to_string(),
        // dynamic_linking: true,
        // executables: true,
        // os_family: Some("unix".to_string()),
        // linker_is_gnu: true,
        // panic_strategy: super::PanicStrategy::Abort,
        // disable_redzone: true,
        // emit_debug_gdb_scripts: false,
        // relocation_model: super::RelocModel::Static,
        // has_rpath: true,
        // pre_link_args: args,
        // position_independent_executables: false,
        // //pre_link_objects_fallback: crt_objects::pre_musl_fallback(),
        // //post_link_objects_fallback: crt_objects::post_musl_fallback(),
        // //crt_objects_fallback: Some(CrtObjectsFallback::Musl),
        // has_elf_tls: true,
        // crt_static_default: true,
        // crt_static_respected: true,
        // crt_static_allows_dylibs: true,
        // mcount: "_mcount".to_string(),
        // ..Default::default()
        os: "rtsmart".into(),
        dynamic_linking: true,
        executables: true,
        families: cvs!["unix"],
        has_rpath: true,
        pre_link_args: args,

        // linker_flavor: LinkerFlavor::Ld,
        // link_script: Some(LINKER_SCRIPT.to_string()),

        crt_static_default: true,

        panic_strategy: PanicStrategy::Abort,
        disable_redzone: true,
        emit_debug_gdb_scripts: false,
        //eh_frame_header: false,

        position_independent_executables: true,
        relro_level: RelroLevel::Full,
        // has_elf_tls: false,
        crt_static_respected: true,
        ..Default::default()
    }
}
