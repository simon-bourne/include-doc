use xtask_base::{
    ci::{StandardVersions, CI},
    generate_open_source_files, CommonCmds,
};

fn main() {
    let code_gen = |check| generate_open_source_files(2021, check);
    CommonCmds::run(
        CI::standard_workflow(StandardVersions::default(), &[]),
        code_gen,
    )
}
