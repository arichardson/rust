use crate::spec::{Cc, CodeModel, LinkerFlavor, Lld, PanicStrategy};
use crate::spec::{RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-pf200:64:64:64:32-p:32:32-i32:32-i64:64-n32-S128"
            .into(),
        llvm_target: "riscv32-unknown-none-elf".into(),
        pointer_width: 32,
        arch: "riscv32".into(),

        options: TargetOptions {
            abi: "ilp32".into(),
            llvm_abiname: "ilp32".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            cpu: "generic-rv32".into(),
            // max_atomic_width: Some(128),
            max_atomic_width: Some(32),
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
