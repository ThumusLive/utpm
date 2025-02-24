use shadow_rs;

fn main() -> shadow_rs::SdResult<()> {
    println!("cargo:rerun-if-changed=build.rs");
    shadow_rs::new()
}
