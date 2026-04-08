use std::fs;
use std::path::Path;

use toml_edit::{DocumentMut, value};

fn parse_cargo_version_from_tag(tag: &str) -> Result<&str, String> {
    let version = tag
        .strip_prefix('v')
        .ok_or_else(|| format!("릴리즈 태그는 'v'로 시작해야 합니다: {tag}"))?;

    if version.is_empty() {
        return Err("버전 문자열이 비어 있습니다".to_string());
    }

    if !version
        .chars()
        .all(|ch| ch.is_ascii_digit() || ch == '.' || ch == '-' || ch == '+')
    {
        return Err(format!("허용되지 않은 버전 문자열입니다: {version}"));
    }

    Ok(version)
}

fn update_manifest_version(manifest_path: &Path, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(manifest_path)?;
    let mut document = content.parse::<DocumentMut>()?;
    document["package"]["version"] = value(version);
    fs::write(manifest_path, document.to_string())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tag = std::env::args()
        .nth(1)
        .ok_or("Usage: prepare-release-version <tag>")?;
    let cargo_version = parse_cargo_version_from_tag(&tag)?;

    update_manifest_version(Path::new("Cargo.toml"), cargo_version)?;

    println!("Updated Cargo.toml version to {cargo_version}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_cargo_version_from_tag, update_manifest_version};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_manifest_path() -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("oshit-release-version-{timestamp}.toml"))
    }

    #[test]
    fn 태그_접두사_v를_제거하고_버전을_반환한다() {
        let version = parse_cargo_version_from_tag("v1.2.3").expect("tag should parse");
        assert_eq!(version, "1.2.3");
    }

    #[test]
    fn v_접두사가_없으면_오류를_반환한다() {
        let error = parse_cargo_version_from_tag("1.2.3").expect_err("tag should fail");
        assert!(error.contains("'v'로 시작"));
    }

    #[test]
    fn cargo_toml의_패키지_버전을_갱신한다() {
        let manifest_path = temp_manifest_path();
        fs::write(
            &manifest_path,
            "[package]\nname = \"oshit\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .expect("manifest should be created");

        update_manifest_version(&manifest_path, "1.2.3").expect("version should update");

        let updated = fs::read_to_string(&manifest_path).expect("manifest should be readable");
        assert!(updated.contains("version = \"1.2.3\""));

        let _ = fs::remove_file(&manifest_path);
    }
}
