use std::fs::File;
use std::io::{self, Read};
use std::ops::DerefMut;
use clap::{App, Arg};

mod run;
mod execbuf;

fn main() -> io::Result<()> {
    let matches = App::new("Rust Shellcode Runner")
        .arg(Arg::with_name("method")
            .short("m")
            .value_name("METHOD")
            .help("Sets the execution method")
            .takes_value(true))
        .arg(Arg::with_name("SHELLCODE")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let mut f = File::open(matches.value_of("SHELLCODE").unwrap())?;

    let mut buf = execbuf::ExecBuf::alloc(f.metadata()?.len() as usize);
    f.read_exact(buf.deref_mut())?;

    match matches.value_of("method").unwrap_or("ret") {
        "tail" => run::tail_call(buf.as_ptr()),
        "ret" => run::ret_call(buf.as_ptr()),
        _ => panic!("Invalid execution method"),
    }

    Ok(())
}
