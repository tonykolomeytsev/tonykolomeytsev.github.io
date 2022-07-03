fn main() {
    std::fs::write("index.html", include_str!("../res/index.html")).unwrap();
}
