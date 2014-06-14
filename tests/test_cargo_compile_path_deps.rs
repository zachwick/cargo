use support::{project,execs,main_file};
use hamcrest::{assert_that,existing_file};
use cargo;
use cargo::util::process;

fn setup() {
}

test!(cargo_compile_simple_path_dep {
    let p = project("foo")
        .file("dep1/Cargo.toml", r#"
            [project]

            name = "dep1"
            version = "0.5.0"
            authors = ["carlhuda@example.com"]

            [[lib]]

            name = "dep1"
        "#)
        .file("dep1/src/dep1.rs", r#"
            pub fn hello() -> &'static str {
                "hello world"
            }
        "#)
        .file("Cargo.toml", format!(r#"
            [project]

            name = "foo"
            version = "0.5.0"
            authors = ["wycats@example.com"]

            [dependencies.dep1]

            version = "0.5.0"
            path = "dep1"

            [[bin]]

            name = "foo"
        "#))
        .file("src/foo.rs", main_file(r#""{}", dep1::hello()"#, ["dep1"]));

    let root = p.root();
    let path_root = root.join("dep1");

    assert_that(p.cargo_process("cargo-compile"),
        execs()
        .with_stdout(format!("Compiling dep1 v0.5.0 (file:{})\nCompiling foo v0.5.0 (file:{})\n",
                             path_root.display(), root.display()))
        .with_stderr(""));

    assert_that(&p.root().join("target/foo"), existing_file());

    assert_that(
      cargo::util::process("foo").extra_path(p.root().join("target")),
      execs().with_stdout("hello world\n"));
})

test!(cargo_compile_transitive_path_dep {
    let p = project("foo")
        .file("dep1/Cargo.toml", r#"
            [project]

            name = "dep1"
            version = "0.5.0"
            authors = ["carlhuda@example.com"]

            [[lib]]

            name = "dep1"

            [dependencies.dep1]

            version = "0.5.0"
            path = "dep2"
        "#)
        .file("dep1/src/dep1.rs", r#"
            extern crate dep2;

            pub fn hello() -> &'static str {
                dep2::hello()
            }
        "#)
        .file("dep1/dep2/Cargo.toml", r#"
            [project]

            name = "dep2"
            version = "0.5.0"
            authors = ["carlhuda@example.com"]

            [[lib]]

            name = "dep2"
        "#)
        .file("dep1/dep2/src/dep2.rs", r#"
            pub fn hello() -> &'static str {
                "hello world"
            }
        "#)
        .file("Cargo.toml", format!(r#"
            [project]

            name = "foo"
            version = "0.5.0"
            authors = ["wycats@example.com"]

            [dependencies.dep1]

            version = "0.5.0"
            path = "dep1"

            [[bin]]

            name = "foo"
        "#))
        .file("src/foo.rs", main_file(r#""{}", dep1::hello()"#, ["dep1"]));

    let root = p.root();
    let dep1 = root.join("dep1");
    let dep2 = dep1.join("dep2");

    assert_that(p.cargo_process("cargo-compile"),
        execs()
        .with_stdout(format!("Compiling dep2 v0.5.0 (file:{})\nCompiling dep1 v0.5.0 (file:{})\nCompiling foo v0.5.0 (file:{})\n",
                             dep2.display(), dep1.display(), root.display()))
        .with_stderr(""));

    assert_that(&p.root().join("target/foo"), existing_file());

    assert_that(
      cargo::util::process("foo").extra_path(p.root().join("target")),
      execs().with_stdout("hello world\n"));
})
