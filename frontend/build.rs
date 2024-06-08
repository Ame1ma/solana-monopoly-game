fn main() {
    let outdir = std::env::var("OUT_DIR").unwrap();
    std::process::Command::new("npx")
        .arg("tailwindcss")
        .arg("-i")
        .arg("./input.css")
        .arg("-o")
        .arg(format!("{outdir}/tailwind.css"))
        .spawn()
        .unwrap();
}
