use std::{env,fs,path::{Path,PathBuf}};
fn profile_dir()->Option<PathBuf>{PathBuf::from(env::var_os("OUT_DIR")?).ancestors().nth(3).map(Path::to_path_buf)}
fn main(){
 let manifest=PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
 let root=manifest.join("..").join("engines").join("zsftp");
 let candidates=[root.join("build").join("Release"),root.join("build")];
 println!("cargo:rerun-if-env-changed=ZSFTP_LIB_DIR");
 let dir=env::var_os("ZSFTP_LIB_DIR").map(PathBuf::from).or_else(||candidates.iter().find(|d|d.join("zsftpcore.dll").is_file()).cloned());
 if let Some(dir)=dir { let dll=dir.join("zsftpcore.dll"); println!("cargo:rustc-link-search=native={}",dir.display()); println!("cargo:rustc-link-lib=dylib=zsftpcore"); if let Some(p)=profile_dir(){for d in [p.clone(),p.join("deps")]{let _=fs::create_dir_all(&d);let _=fs::copy(&dll,d.join("zsftpcore.dll"));}} }
 else { panic!("ZSFTP core is not built. Run: cmake -S engines/zsftp -B engines/zsftp/build -DRARFTP_BUILD_LIBRARY=ON -DRARFTP_BUILD_TESTS=OFF && cmake --build engines/zsftp/build --config Release"); }
 tauri_build::build();
}