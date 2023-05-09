extern crate cc;
extern crate cmake;

fn build_sentencepiece(builder: &mut cc::Build) {
    let dst = cmake::Config::new("sentencepiece").pic(true).build();
    println!(
        "cargo:rustc-link-search=native={}",
        //dst.join("build").join("src").display()
        dst.join("lib64").display()
    );
    println!("cargo:rustc-link-lib=static=sentencepiece");
    builder.include("sentencepiece/src");
}

fn main() {
    let mut builder = cc::Build::new();
    build_sentencepiece(&mut builder);
    builder
        .file("sentencepiece_wrap.cc")
        .cpp(true)
        .flag_if_supported("-std=c++17")
        .opt_level(2)
        .pic(true)
        .compile("sentencepiece_wrap");
    println!("cargo:rerun-if-changed=sentencepiece_wrap.cc");
    //println!("cargo:rerun-if-changed=sentencepiece_wrap.h");
    println!("cargo:rerun-if-changed=build.rs");
}
