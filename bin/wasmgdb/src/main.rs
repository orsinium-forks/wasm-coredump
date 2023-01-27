use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use wasmgdb_ddbug_parser as ddbug_parser;

mod commands;
mod memory;
mod repl;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Source WebAssembly module
    source: String,
    /// Coredump
    coredump: Option<String>,
}

pub(crate) type BoxError = Box<dyn std::error::Error>;

pub fn main() -> Result<(), BoxError> {
    env_logger::init();

    let args = Args::parse();
    let source_filename = args.source;

    let coredump = if let Some(coredump_filename) = args.coredump {
        let mut coredump = Vec::new();
        {
            let mut file = File::open(coredump_filename).expect("File not found");
            file.read_to_end(&mut coredump)
                .expect("Error while reading file");
        }

        let coredump_wasm = wasm_parser::parse(&coredump)
            .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
        let coredump_wasm = core_wasm_ast::traverse::WasmModule::new(Arc::new(coredump_wasm));
        let coredump = coredump_wasm.get_coredump()?;
        Some(coredump)
    } else {
        None
    };

    let ctx = ddbug_parser::File::parse(source_filename.clone()).unwrap();
    let ddbug = ddbug_parser::FileHash::new(ctx.file());

    let mut source = Vec::new();
    {
        let mut file = File::open(source_filename).expect("File not found");
        file.read_to_end(&mut source)
            .expect("Error while reading file");
    }

    let source = wasm_parser::parse(&source)
        .map_err(|err| format!("failed to parse Wasm module: {}", err))?;
    let source = core_wasm_ast::traverse::WasmModule::new(Arc::new(source));

    repl::repl(coredump, &source, ddbug)
}
