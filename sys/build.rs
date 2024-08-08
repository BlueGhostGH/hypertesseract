#[cfg(windows)]
fn main()
{
    let _leptonica = vcpkg::Config::new().find_package("leptonica").unwrap();
    let _tesseract = vcpkg::Config::new().find_package("tesseract").unwrap();
}

#[cfg(target_os = "linux")]
fn main()
{
    let leptonica = pkg_config::Config::new()
        .atleast_version("1.78")
        .probe("lept")
        .unwrap();
    let tesseract = pkg_config::Config::new()
        .atleast_version("4.1")
        .probe("tesseract")
        .unwrap();

    println!(
        "cargo::rustc-link-search=native={:?}",
        leptonica.link_paths[0]
    );
    println!(
        "cargo::rustc-link-search=native={:?}",
        tesseract.link_paths[0]
    );
    println!("cargo::rustc-link-lib={}", leptonica.libs[0]);
    println!("cargo::rustc-link-lib={}", tesseract.libs[0]);
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn main()
{
    panic!("Unsupported target OS!");
}
