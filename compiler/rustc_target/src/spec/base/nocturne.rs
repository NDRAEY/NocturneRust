use crate::spec::crt_objects;
use crate::spec::{Cc, LinkerFlavor, Lld, RelocModel, StackProbeType, TargetOptions};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "nocturne".into(),
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        stack_probes: StackProbeType::Inline,
        relocation_model: RelocModel::Static,
        pre_link_objects: crt_objects::pre_nocturne(),
        post_link_objects: crt_objects::post_nocturne(),
        ..Default::default()
    }
}
