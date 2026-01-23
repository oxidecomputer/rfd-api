fn main() {
    println!("cargo:rerun-if-changed=../rfd-model/migrations");
}
