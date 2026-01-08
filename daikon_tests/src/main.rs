use colored::Colorize;

// Given a Rust source file name, splice out the name of the file
// by excluding the .rs extension and any preceding directory names.
pub fn get_output_name(s: String) -> String {
    let end = match s.rfind(".") {
        // .rs
        None => panic!("no . at the end of input file name"),
        Some(end) => end,
    };
    let mut start = match s.rfind("/") {
        // .../<crate>.rs
        None => 0,
        Some(slash) => slash + 1,
    };
    let mut res = String::from("");
    while start < end {
        res.push_str(&format!("{}", s.chars().nth(start).unwrap()));
        start += 1;
    }
    return res;
}

// iterate through "./tests" and run rustc +daikon for files, cargo +daikon for multi-file tests in subdirectories
fn run_daikon_rustc_pp_tests() {
    let in_ci = std::env::var("CROSAE_CI").is_ok();
    let test_path_str =
        if in_ci {
            "/checkout/obj/daikon_tests/test/"
        } else {
            "./test/"
        };
    let rustc_path =
        if in_ci {
            "/checkout/obj/build/host/stage1/bin/rustc" // we can use stage2 probably in ci
        } else {
            "../../build/host/stage1/bin/rustc"
        };
    println!("{rustc_path}");

    let test_path = std::fs::canonicalize(std::path::Path::new(&test_path_str)).unwrap();

    // how to make each one of these a test w/o a new function?
    for entry in std::fs::read_dir(test_path.clone()).unwrap() {
        let entry = entry.unwrap();
        let path = std::fs::canonicalize(entry.path()).unwrap();
        if path.is_dir() {
            // set current_dir to canonicalize(<dir>) in Command and do cargo +daikon build
        } else {
            // set current_dir to canonicalize(test_path.clone()) and execute rustc +daikon

            let path_str = path.to_str().unwrap();
            if !path_str.ends_with("rs") {
                continue;
            } else {
                let output_name = get_output_name(String::from(path_str));
                println!("Running test {}", output_name);

                std::process::Command::new(&rustc_path)
                    .arg(path_str)
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .current_dir(&test_path)
                    .status()
                    .expect("failed to execute c-rosae");

                // read expected/actual pp to String
                let pp_path = format!("{test_path_str}{}{}", output_name, ".pp");
                let pp_as_path = std::path::Path::new(&pp_path);
                let pp_as_path_buf = std::fs::canonicalize(pp_as_path).unwrap();
                let actual = std::fs::read_to_string(&pp_as_path_buf).unwrap();
                let pp_expected_path = format!("{test_path_str}{}-expected{}", output_name, ".pp");
                let pp_expected_as_path = std::path::Path::new(&pp_expected_path);
                let pp_expected_as_path_buf = std::fs::canonicalize(pp_expected_as_path).unwrap();
                let expected = std::fs::read_to_string(&pp_expected_as_path_buf).unwrap();

                // remove junk
                let exec_path = format!("{test_path_str}{}", output_name);
                std::fs::remove_file(std::path::Path::new(&exec_path)).unwrap();
                let decls_path = format!("{test_path_str}{}.decls", output_name);
                std::fs::remove_file(std::path::Path::new(&decls_path)).unwrap();
                let dtrace_path = format!("{test_path_str}{}.dtrace", output_name);
                std::fs::remove_file(std::path::Path::new(&dtrace_path)).unwrap();
                let pp_path = format!("{test_path_str}{}.pp", output_name);
                std::fs::remove_file(std::path::Path::new(&pp_path)).unwrap();

                // check
                assert_eq!(expected, actual);
                println!("{}", "Pass".green());
            }
        }
    }

    println!("\n{}", "All tests passed".green());
}

fn main() {
    run_daikon_rustc_pp_tests();
}
