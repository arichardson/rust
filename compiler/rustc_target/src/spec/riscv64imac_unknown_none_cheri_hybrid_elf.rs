use crate::spec::{Cc, CodeModel, LinkerFlavor, Lld, PanicStrategy};
use crate::spec::{RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-pf200:128:128:128:64-p:64:64-i64:64-i128:128-n64-S128"
            .into(),
        llvm_target: "riscv64-unknown-none-elf".into(),
        pointer_width: 64,
        arch: "riscv64".into(),

        options: TargetOptions {
            abi: "lp64".into(),
            llvm_abiname: "lp64".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            cpu: "generic-rv64".into(),
            // max_atomic_width: Some(128),
            max_atomic_width: Some(64),
            features: "+m,+a,+c,+xcheri,-cap-mode,-relax,-save-restore,-xcheri-rvc".into(),
            executables: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            code_model: Some(CodeModel::Medium),
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
