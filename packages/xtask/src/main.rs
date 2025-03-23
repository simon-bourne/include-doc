use xtask_base::{
    ci::{StandardVersions, CI},
    generate_open_source_files, CommonCmds,
};

fn main() {
    let code_gen = |check| generate_open_source_files(2021, check);
    CommonCmds::run(
        CI::standard_workflow(
            StandardVersions {
                rustc_stable_version: "1.85.1",
                rustc_nightly_version: "nightly-2025-03-15",
                udeps_version: "0.1.55",
            },
            &[],
        ),
        code_gen,
    )
}
