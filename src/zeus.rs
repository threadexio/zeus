mod config;
mod log;

mod build;
mod sync;

use args::Args;
use bollard::Docker;
use getopts::Occur;

use std::env;
use std::fs::create_dir_all;
use std::process::{exit, Command};

#[tokio::main]
async fn main() {
    let _args: Vec<String> = env::args().collect();
    let mut args = Args::new(config::PROGRAM_NAME, config::PROGRAM_DESC);

    args.flag("S", "sync", "Sync packages");
    args.flag("B", "build-builder", "Build the builder image");

    args.flag("u", "upgrade", "Upgrade packages before build");
    args.option(
        "p",
        "packages",
        "Packages to perform operations on",
        "<name,name,...>",
        Occur::Multi,
        None,
    );

    args.flag("h", "help", "This help menu");
    args.flag("v", "verbose", "Be verbose");
    args.flag("", "force", "Ignore all warnings");

    args.option(
        "",
        "color",
        "Colorize the output",
        "<when>",
        Occur::Optional,
        None,
    );

    args.option(
        "",
        "buildargs",
        "Extra arguments for makepkg",
        "args",
        Occur::Optional,
        None,
    );

    args.option(
        "",
        "builddir",
        "Package build directory",
        "<path>",
        Occur::Optional,
        None,
    );

    args.option(
        "",
        "image",
        "Builder image name",
        "<name:tag>",
        Occur::Optional,
        None,
    );

    args.option(
        "",
        "imagearchive",
        "Builder image build archive",
        "<path>",
        Occur::Optional,
        None,
    );

    args.option(
        "",
        "dockerfile",
        "Builder image dockerfile",
        "<path>",
        Occur::Optional,
        None,
    );

    if _args.len() == 1 {
        eprintln!("{}", args.full_usage());
        exit(0);
    }

    match args.parse(&_args) {
        Err(e) => {
            eprintln!("{}", e.to_string());
            exit(1);
        }
        _ => {}
    }

    if args.value_of::<bool>("help").unwrap() {
        eprintln!("{}", args.full_usage());
        exit(1);
    }

    let mut logger = log::Logger::new(
        log::Stream::Stdout,
        match &(args
            .value_of::<String>("color")
            .unwrap_or("auto".to_owned()))[..]
        {
            "always" => log::ColorChoice::Always,
            "never" => log::ColorChoice::Never,
            _ => log::ColorChoice::Auto,
        },
    );

    let docker = match Docker::connect_with_local_defaults() {
        Ok(v) => v,
        Err(e) => {
            logger.v(
                log::Level::Error,
                "docker",
                format!("Unable to connect to daemon: {}", e),
            );
            exit(1);
        }
    };

    let cfg = config::Config {
        verbose: args.value_of("verbose").unwrap_or(false),
        force: args.value_of("force").unwrap_or(false),
        upgrade: args.value_of("upgrade").unwrap_or(false),

        builder_archive: args
            .value_of("imagearchive")
            .unwrap_or("builder.tar.gz".to_owned()),
        builder_dockerfile: args
            .value_of("dockerfile")
            .unwrap_or("Dockerfile".to_owned()),
        builder_image: args
            .value_of("image")
            .unwrap_or("zeus-builder:latest".to_owned()),

        packages: args.values_of("packages").unwrap_or(vec![]),
        build_dir: args
            .value_of("builddir")
            .unwrap_or("/var/cache/aur".to_owned()),
        build_args: args
            .value_of::<String>("buildargs")
            .unwrap_or("".to_owned())
            .split(" ")
            .map(|e| e.to_owned())
            .collect(),
    };

    // TODO: Implement a locking mechanism to ensure only one instance is active at any time

    if args.value_of::<bool>("sync").unwrap() {
        sync::sync(logger, docker, cfg).await;
    } else if args.value_of::<bool>("build-builder").unwrap() {
        build::build(logger, docker, cfg).await;
    } else {
        logger.v(
            log::Level::Error,
            config::PROGRAM_NAME,
            "No operation specified! See --help",
        );
    }
}
