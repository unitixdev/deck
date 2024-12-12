load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_tarball")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "content-api",
    srcs = glob(["src/**/*.rs"]),
    data = glob(["migrations/**/*.sql"]),
    edition = "2021",
    proc_macro_deps = all_crate_deps(proc_macro = True),
    deps = all_crate_deps(normal = True),
)

pkg_tar(
    name = "layer",
    srcs = [":content-api"],
)

oci_image(
    name = "image",
    base = "@distroless_cc",
    entrypoint = ["/content-api"],
    tars = [":layer"],
)

oci_tarball(
    name = "image.tar",
    image = ":image",
    repo_tags = ["content-api:latest"],
)