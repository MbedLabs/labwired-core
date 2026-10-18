// LabWired - Firmware Simulation Platform
// Copyright (C) 2026 Andrii Shylenko
//
// This software is released under the MIT License.
// See the LICENSE file in the project root for full license information.

//! `--log-timestamps`: the default run stays byte-silent about time (no `[t=`
//! prefix, no `result.json` keys); `virtual` prefixes are byte-stable across
//! runs; `both` adds real time that is present but explicitly not stable; and
//! `labwired run` carries the same prefixes as `labwired test`.

use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// One `labwired test` run over the determinism fixture.
///
/// `mode` is the `--log-timestamps` value; `None` omits the flag entirely so
/// the default environment stays representative. `log` sets `RUST_LOG` only
/// for tests that need the tracing INFO lines present. The script is written
/// with ABSOLUTE firmware/system paths because script `inputs` resolve
/// relative to the script file, and this one lives in a temp dir.
fn run(mode: Option<&str>, log: Option<&str>) -> (std::process::Output, serde_json::Value) {
    let root = workspace_root();
    let dir = labwired_cli::test_support::unique_temp_dir(&format!(
        "labwired-logts-{}",
        mode.unwrap_or("default")
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");

    let firmware = root
        .join("tests/fixtures/uart-ok-thumbv7m.elf")
        .canonicalize()
        .expect("firmware fixture");
    let system = root
        .join("configs/systems/ci-fixture-uart1.yaml")
        .canonicalize()
        .expect("system fixture");
    let script_path = dir.join("script.yaml");
    std::fs::write(
        &script_path,
        format!(
            "schema_version: \"1.0\"\ninputs:\n  firmware: \"{}\"\n  system: \"{}\"\nlimits:\n  max_steps: 1000\nassertions: []\n",
            firmware.display(),
            system.display()
        ),
    )
    .expect("write script");

    let out_dir = dir.join("out");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_labwired"));
    cmd.arg("test")
        .arg("--script")
        .arg(&script_path)
        .arg("--no-uart-stdout")
        .arg("--output-dir")
        .arg(&out_dir);
    if let Some(mode) = mode {
        cmd.arg("--log-timestamps").arg(mode);
    }
    if let Some(filter) = log {
        cmd.env("RUST_LOG", filter);
    }
    let out = cmd.output().expect("spawn labwired");

    let result_json = std::fs::read_to_string(out_dir.join("result.json")).unwrap_or_else(|e| {
        panic!(
            "run failed to produce result.json (exit {:?}): {e}\nstderr:\n{}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        )
    });
    let result: serde_json::Value = serde_json::from_str(&result_json).expect("parse result.json");
    // Scratch dir removed only after result.json has been read and parsed. A
    // failed run panics above and intentionally leaves the dir in place so its
    // stderr and leftover artifacts stay readable for diagnosis; the returned
    // `Output` keeps its captured stderr either way.
    let _ = std::fs::remove_dir_all(&dir);
    (out, result)
}

fn ts_lines(stderr: &str) -> Vec<&str> {
    stderr.lines().filter(|l| l.starts_with("[t=")).collect()
}

/// The verdict banner embeds the run's wall duration (`{:.2}s`), which is
/// explicitly NOT stable under load (0.00s–0.02s observed). Mask exactly that
/// trailing `\d+\.\d{2}s` token so the rest of the line — virtual prefix,
/// counts, subject, steps — is still compared byte-for-byte.
fn normalize_banner_duration(line: &str) -> String {
    match line.rsplit_once(' ') {
        Some((head, token)) if is_centisecond_duration(token) => format!("{head} <DUR>s"),
        _ => line.to_string(),
    }
}

fn is_centisecond_duration(token: &str) -> bool {
    let Some(secs) = token.strip_suffix('s') else {
        return false;
    };
    let Some((int_part, frac)) = secs.split_once('.') else {
        return false;
    };
    !int_part.is_empty()
        && int_part.bytes().all(|b| b.is_ascii_digit())
        && frac.len() == 2
        && frac.bytes().all(|b| b.is_ascii_digit())
}

#[test]
fn default_runs_have_no_timestamp_prefix_and_no_result_keys() {
    let (out, result) = run(None, None);
    assert!(
        out.status.success(),
        "default run must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        ts_lines(&stderr).is_empty(),
        "no `--log-timestamps` means no `[t=` line at all; stderr:\n{stderr}"
    );
    assert!(
        result.get("virtual_time_ns").is_none(),
        "default result.json must not gain virtual_time_ns"
    );
    assert!(
        result.get("host_time_ns").is_none(),
        "default result.json must not gain host_time_ns"
    );
}

#[test]
fn virtual_prefixes_are_stable_across_runs() {
    let (a, ra) = run(Some("virtual"), Some("info"));
    let (b, rb) = run(Some("virtual"), Some("info"));
    assert!(
        a.status.success() && b.status.success(),
        "virtual runs must succeed; stderr a:\n{}\nstderr b:\n{}",
        String::from_utf8_lossy(&a.stderr),
        String::from_utf8_lossy(&b.stderr)
    );

    let sa = String::from_utf8_lossy(&a.stderr).into_owned();
    let sb = String::from_utf8_lossy(&b.stderr).into_owned();
    let la = ts_lines(&sa);
    let lb = ts_lines(&sb);

    assert!(
        !la.is_empty(),
        "virtual mode must prefix at least the banner line; stderr:\n{sa}"
    );
    assert!(
        la.iter().all(|l| l.starts_with("[t=virt=")),
        "every `[t=` line must start with the virtual prefix; lines: {la:?}"
    );
    let normalized = |lines: &[&str]| -> Vec<String> {
        lines.iter().map(|l| normalize_banner_duration(l)).collect()
    };
    assert_eq!(
        normalized(&la),
        normalized(&lb),
        "virtual event lines must be byte-identical across runs (banner wall \
         duration masked); run A: {la:?}\nrun B: {lb:?}"
    );

    // Pin the formatter wiring, not just the `eventln!` macro: an INFO tracing
    // line must carry the prefix too (RUST_LOG=info guarantees at least one).
    assert!(
        la.iter()
            .any(|l| l.starts_with("[t=virt=") && l.contains("INFO")),
        "the custom tracing formatter must prefix INFO lines; lines: {la:?}"
    );

    let va = ra["virtual_time_ns"]
        .as_u64()
        .expect("virtual mode must write virtual_time_ns");
    let vb = rb["virtual_time_ns"]
        .as_u64()
        .expect("virtual mode must write virtual_time_ns");
    assert_eq!(va, vb, "virtual_time_ns is deterministic; A={va} B={vb}");
    assert!(
        ra.get("host_time_ns").is_none(),
        "virtual mode must not write host_time_ns"
    );
}

#[test]
fn both_mode_adds_real_time_that_is_present_but_not_stable() {
    let (a, ra) = run(Some("both"), Some("info"));
    let (b, rb) = run(Some("both"), Some("info"));
    assert!(
        a.status.success() && b.status.success(),
        "both-mode runs must succeed; stderr a:\n{}\nstderr b:\n{}",
        String::from_utf8_lossy(&a.stderr),
        String::from_utf8_lossy(&b.stderr)
    );

    let sa = String::from_utf8_lossy(&a.stderr).into_owned();
    let sb = String::from_utf8_lossy(&b.stderr).into_owned();
    let la = ts_lines(&sa);
    assert!(!la.is_empty(), "both mode must prefix lines; stderr:\n{sa}");
    assert!(
        la.iter().all(|l| l.contains("|real=+")),
        "every `[t=` line must carry the real component; lines: {la:?}"
    );
    assert!(
        ra["host_time_ns"].as_u64().expect("host_time_ns") > 0,
        "both mode must write a positive host_time_ns"
    );
    assert!(
        rb["host_time_ns"].as_u64().expect("host_time_ns") > 0,
        "both mode must write a positive host_time_ns"
    );

    // The real component is explicitly not comparable, so full lines cannot be
    // compared; the virtual component must still not move between runs.
    let virt = |s: &str| -> Vec<String> {
        ts_lines(s)
            .iter()
            .map(|l| l.split('|').next().unwrap().to_string())
            .collect()
    };
    assert_eq!(
        virt(&sa),
        virt(&sb),
        "the virtual component must be identical across runs"
    );
}

/// `labwired run` must carry the same prefixes as `labwired test`: the flag
/// was wired into both commands, and only `test` is covered above.
#[test]
fn run_command_prefixes_too() {
    let root = workspace_root();
    let run_once = |extra: &[&str]| -> std::process::Output {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_labwired"));
        cmd.arg("run")
            .arg("--chip")
            .arg(root.join("configs/chips/nrf54l15.yaml"))
            .arg("--firmware")
            .arg(root.join("tests/fixtures/nrf54l15-smoke.elf"))
            .arg("--max-steps")
            .arg("2000000")
            .arg("--batched");
        cmd.args(extra).output().expect("spawn labwired run")
    };

    let out = run_once(&["--log-timestamps", "virtual"]);
    assert!(
        out.status.success(),
        "run must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let lines: Vec<&str> = stderr
        .lines()
        .filter(|l| l.starts_with("[t=virt="))
        .collect();
    assert!(
        !lines.is_empty(),
        "run with --log-timestamps virtual must prefix its event lines; stderr:\n{stderr}"
    );
    assert!(
        lines.iter().any(|l| {
            l.strip_prefix("[t=virt=")
                .and_then(|rest| rest.split(']').next())
                .and_then(|ts| ts.strip_suffix('s'))
                .and_then(|n| n.parse::<f64>().ok())
                .is_some_and(|secs| secs > 0.0)
        }),
        "at least one run event line must carry a NON-ZERO virtual time \
         (cycles have advanced by the final `[batched]` line); lines: {lines:?}"
    );

    let plain = run_once(&[]);
    assert!(plain.status.success(), "plain run must succeed");
    let plain_stderr = String::from_utf8_lossy(&plain.stderr);
    assert!(
        !plain_stderr.lines().any(|l| l.starts_with("[t=")),
        "without the flag, `run` must not prefix any line; stderr:\n{plain_stderr}"
    );
}
