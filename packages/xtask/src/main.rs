use xtask_base::{ci::CI, generate_open_source_files, CommonCmds};

fn main() {
    let code_gen = |check| generate_open_source_files(2021, check);
    CommonCmds::run(CI::standard_workflow(), code_gen)
}
