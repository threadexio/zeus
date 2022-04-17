mod config;
mod error;
mod log;

mod build;
mod sync;
mod version;

use error::ZeusError;

use args::Args;
use bollard::Docker;
use getopts::Occur;

use std::env;
use std::process::exit;

#[tokio::main]
async fn main() {
    let _args: Vec<String> = env::args().collect();
    let mut args = Args::new(config::PROGRAM_NAME, config::PROGRAM_DESC);

    let defaults = config::Config::default();

    args.flag("S", "sync", "Sync packages");
    args.flag("B", "build-builder", "Build the builder image");
    args.flag("V", "version", "");

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
        Some("auto".to_owned()),
    );

    args.option(
        "",
        "archive",
        "Builder image archive (only with -B)",
        "<path>",
        Occur::Optional,
        Some(defaults.builder.archive),
    );

    args.option(
        "",
        "dockerfile",
        "Builder image dockerfile (only with -B)",
        "<path>",
        Occur::Optional,
        Some(defaults.builder.dockerfile),
    );

    args.option(
        "",
        "image",
        "Builder image name",
        "<name:tag>",
        Occur::Optional,
        Some(defaults.builder.image),
    );

    args.option(
        "",
        "name",
        "Builder container name",
        "<name>",
        Occur::Optional,
        Some(defaults.builder.name),
    );

    args.option(
        "",
        "builddir",
        "Package build directory",
        "<path>",
        Occur::Optional,
        Some(defaults.build_dir),
    );

    args.option(
        "",
        "buildargs",
        "Extra arguments for makepkg",
        "<args>",
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
        exit(0);
    }

    let mut logger = log::Logger::new(
        log::Stream::Stdout,
        match &(args.value_of::<String>("color").unwrap())[..] {
            "always" => log::ColorChoice::Always,
            "never" => log::ColorChoice::Never,
            _ => log::ColorChoice::Auto,
        },
    );

    logger.verbose = args.value_of("verbose").unwrap_or(false);

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
        packages: args.values_of("packages").unwrap_or(vec![]),

        force: args.value_of("force").unwrap_or(false),
        upgrade: args.value_of("upgrade").unwrap_or(false),

        builder: config::Builder {
            archive: args.value_of("archive").unwrap(),
            dockerfile: args.value_of("dockerfile").unwrap(),
            image: args.value_of("image").unwrap(),
            name: args.value_of("name").unwrap(),
        },

        build_dir: args.value_of("builddir").unwrap(),
        build_args: args
            .value_of::<String>("buildargs")
            .unwrap_or("".to_owned())
            .split(" ")
            .map(|e| e.to_owned())
            .collect(),
    };

    // TODO: Implement a locking mechanism to ensure only one instance is active at any time

    let result: Result<(), ZeusError>;
    if args.value_of::<bool>("sync").unwrap() {
        result = sync::sync(&mut logger, docker, cfg).await;
    } else if args.value_of::<bool>("build-builder").unwrap() {
        result = build::build(&mut logger, docker, cfg).await;
    } else if args.value_of::<bool>("version").unwrap() {
        result = version::version(docker).await;
    } else {
        logger.v(
            log::Level::Error,
            config::PROGRAM_NAME,
            "No operation specified! See --help",
        );
        return;
    }

    match result {
        Ok(_) => {}
        Err(e) => {
            logger.v(log::Level::Error, e.facility, e.data);
            exit(1);
        }
    }
}
