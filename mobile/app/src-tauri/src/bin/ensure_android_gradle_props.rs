use std::{
    fs,
    io,
    path::{Path, PathBuf},
};

fn find_gradle_properties(cwd: &Path) -> Option<PathBuf> {
    let candidates = [
        cwd.join("src-tauri").join("gen").join("android").join("gradle.properties"),
        cwd.join("gen").join("android").join("gradle.properties"),
    ];

    candidates.into_iter().find(|p| p.exists())
}

fn find_android_app_build_gradle(cwd: &Path) -> Option<PathBuf> {
    let candidates = [
        cwd.join("src-tauri")
            .join("gen")
            .join("android")
            .join("app")
            .join("build.gradle.kts"),
        cwd.join("gen")
            .join("android")
            .join("app")
            .join("build.gradle.kts"),
    ];

    candidates.into_iter().find(|p| p.exists())
}

fn patch_key(lines: &mut Vec<String>, key: &str, value: &str) {
    let mut seen = false;
    let prefix = format!("{key}=");

    lines.retain_mut(|line| {
        if line.trim_start().starts_with(&prefix) {
            if seen {
                return false;
            }
            *line = format!("{key}={value}");
            seen = true;
        }
        true
    });

    if !seen {
        lines.push(format!("{key}={value}"));
    }
}

fn ensure_release_signed_for_testers(contents: &str) -> Option<String> {
    let mut lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();

    // Insert signingConfig into the release build type if not already present.
    // We keep this minimal and idempotent.
    let mut in_release = false;
    let mut release_has_signing = false;
    let mut release_open_line_idx: Option<usize> = None;

    for (idx, line) in lines.iter().enumerate() {
        if line.contains("getByName(\"release\")") {
            in_release = true;
            release_open_line_idx = Some(idx);
            continue;
        }

        if in_release {
            if line.contains("signingConfig")
                && line.contains("signingConfigs")
                && line.contains("getByName(\"debug\")")
            {
                release_has_signing = true;
            }

            // End of the release block (best-effort; matches the exact file structure).
            if line.trim() == "}" {
                in_release = false;
            }
        }
    }

    if release_has_signing {
        return None;
    }

    let Some(open_idx) = release_open_line_idx else {
        return None;
    };

    let open_line = &lines[open_idx];
    let indent: String = open_line.chars().take_while(|c| c.is_whitespace()).collect();
    let insert_line = format!("{indent}    signingConfig = signingConfigs.getByName(\"debug\")");

    lines.insert(open_idx + 1, insert_line);

    Some(lines.join("\n") + "\n")
}

fn main() -> io::Result<()> {
    let cwd = std::env::current_dir()?;

    if let Some(path) = find_gradle_properties(&cwd) {
        let original = fs::read_to_string(&path)?;
        let eol = if original.contains("\r\n") { "\r\n" } else { "\n" };

        let mut lines: Vec<String> = original.lines().map(|l| l.to_string()).collect();

        patch_key(&mut lines, "org.gradle.warning.mode", "none");
        patch_key(
            &mut lines,
            "android.javaCompile.suppressSourceTargetDeprecationWarning",
            "true",
        );
        patch_key(&mut lines, "org.gradle.problems.report", "false");

        let mut updated = lines.join(eol);
        if !updated.ends_with(eol) {
            updated.push_str(eol);
        }

        if updated != original {
            fs::write(&path, updated)?;
        }
    }

    if let Some(path) = find_android_app_build_gradle(&cwd) {
        let original = fs::read_to_string(&path)?;
        let eol = if original.contains("\r\n") { "\r\n" } else { "\n" };

        if let Some(mut patched) = ensure_release_signed_for_testers(&original) {
            if eol == "\r\n" {
                patched = patched.replace("\n", "\r\n");
            }
            if patched != original {
                fs::write(&path, patched)?;
            }
        }
    }

    Ok(())
}
