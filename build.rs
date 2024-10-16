/// Pre-installed OpenCASCADE library will be checked for compatibility using semver rules.
const OCCT_VERSION: (u8, u8) = (7, 8);

/// The list of used OpenCASCADE modules which libraries needs to be linked with.
const OCCT_MODULES: &[&str] = &[
    "FoundationClasses",
    "ModelingData",
    "ModelingAlgorithms",
    //"Visualization",
    //"ApplicationFramework",
    "DataExchange",
    //"DETools",
    //"Draw",
];

fn main() {
    let target = std::env::var("TARGET").expect("No TARGET environment variable defined");
    let is_windows = target.to_lowercase().contains("windows");
    let is_windows_gnu = target.to_lowercase().contains("windows-gnu");

    let occt_config = OcctConfig::detect();

    println!(
        "cargo:rustc-link-search=native={}",
        occt_config.library_dir.to_str().unwrap()
    );

    let lib_type = if occt_config.is_dynamic {
        "dylib"
    } else {
        "static"
    };

    for module_name in OCCT_MODULES {
        if let Some(module) = occt_config.modules.get(*module_name) {
            for lib in &module.libs {
                println!("cargo:rustc-link-lib={lib_type}={lib}");
            }
        } else {
            panic!("Required OpenCASCADE module '{module_name}' not found!");
        }
    }

    if is_windows {
        println!("cargo:rustc-link-lib=dylib=user32");
    }

    let mut build = cpp_build::Config::new();

    if is_windows_gnu {
        build.define("OCC_CONVERT_SIGNALS", Some("TRUE"));
    }

    build
        .flag_if_supported("-std=c++11")
        .define("_USE_MATH_DEFINES", Some("TRUE"))
        .include(occt_config.include_dir)
        .build("src/lib.rs");

    println!("cargo:rerun-if-changed=src/*.rs");
    println!("cargo:rerun-if-changed=src/math/*.rs");
    println!("cargo:rerun-if-changed=src/brep/*.rs");
}

#[derive(Default)]
struct OcctModule {
    libs: Vec<String>,
}

struct OcctConfig {
    include_dir: std::path::PathBuf,
    library_dir: std::path::PathBuf,
    is_dynamic: bool,
    modules: std::collections::HashMap<String, OcctModule>,
}

impl OcctConfig {
    /// Find OpenCASCADE library using cmake
    fn detect() -> Self {
        println!("cargo:rerun-if-env-changed=DEP_OCCT_ROOT");

        let dst =
            std::panic::catch_unwind(|| cmake::Config::new("occt").register_dep("occt").build());

        let dst = dst.expect("Pre-installed OpenCASCADE library not found.");

        let cfg = std::fs::read_to_string(dst.join("share").join("occ_info.txt"))
            .expect("Something went wrong when detecting OpenCASCADE library.");

        let mut version_major: Option<u8> = None;
        let mut version_minor: Option<u8> = None;
        let mut include_dir: Option<std::path::PathBuf> = None;
        let mut library_dir: Option<std::path::PathBuf> = None;
        let mut modules: std::collections::HashMap<String, OcctModule> = Default::default();
        let mut is_dynamic: bool = false;

        for line in cfg.lines() {
            if let Some((var, val)) = line.split_once('=') {
                match var {
                    "VERSION_MAJOR" => version_major = val.parse().ok(),
                    "VERSION_MINOR" => version_minor = val.parse().ok(),
                    "INCLUDE_DIR" => include_dir = val.parse().ok(),
                    "LIBRARY_DIR" => library_dir = val.parse().ok(),
                    "BUILD_SHARED_LIBS" => is_dynamic = val == "ON",
                    _ => {
                        if let Some((pfx, stm, sfx)) = var.split_once('_').and_then(|(pfx, rst)| {
                            rst.split_once('_').map(|(stm, sfx)| (pfx, stm, sfx))
                        }) {
                            if pfx == "MODULE" && sfx == "LIBS" {
                                modules
                                    .entry(stm.to_string())
                                    .or_insert_with(|| OcctModule::default())
                                    .libs = val.split(';').map(|lib| lib.to_string()).collect();
                            }
                        }
                    }
                }
            }
        }

        if let (Some(version_major), Some(version_minor), Some(include_dir), Some(library_dir)) =
            (version_major, version_minor, include_dir, library_dir)
        {
            if version_major != OCCT_VERSION.0 || version_minor < OCCT_VERSION.1 {
                panic!("Pre-installed OpenCASCADE library found but version is not met (found {}.{} but {}.{} required).",
                       version_major, version_minor, OCCT_VERSION.0, OCCT_VERSION.1);
            }

            Self {
                include_dir,
                library_dir,
                is_dynamic,
                modules,
            }
        } else {
            panic!("OpenCASCADE library found but something wrong with config.");
        }
    }
}
