fn main() {
    if std::env::var("PROFILE").unwrap() == "release" {
        let _ = embed_resource::compile("app.rc", embed_resource::NONE);
    }
}
