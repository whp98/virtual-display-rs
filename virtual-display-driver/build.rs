fn main() {
    let mut winres = winres::WindowsResource::new();

    // as much as I don't like setting this here, because it belongs to the logger crate
    // I want my own resource info for this crate. There's a linker error unless we do it
    // all in one fell swoop. This is the only option for now
    //
    // For location of the content, see winlog/res/eventmsgs.rc
    winres
        .append_rc_content(
            r#"
/* Do not edit this file manually.
This file is autogenerated by windmc.  */


// Country: United States
// Language: English
#pragma code_page(437)
LANGUAGE 0x9, 0x1
1 MESSAGETABLE "res/MSG00409.bin"
    "#,
        )
        .compile()
        .unwrap();

    // need linked c runtime for umdf includes
    println!("cargo:rustc-link-lib=static=ucrt");
    //println!("cargo:rustc-link-lib=static=vcruntime");
}
