fn main() {
    windows::build! {
        Windows::Win32::System::Memory::VirtualAlloc
    };
    cc::Build::new()
        .file("src/run.asm")
        .compile("run.lib");
}