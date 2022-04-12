mod config;
mod log;
mod operation;

use args::Args;
use bollard::Docker;
use getopts::Occur;
use std::env;
use std::process::exit;

#[tokio::main]
async fn main() {
    let _args: Vec<String> = env::args().collect();
    let mut args = Args::new(config::PROGRAM_NAME, config::PROGRAM_DESC);

    args.flag("S", "sync", "Sync packages");
    args.flag("B", "build-builder", "Build the builder image");

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
        "builddir",
        "Package build directory",
        "<path>",
        Occur::Optional,
        Some("/var/cache/aur".to_owned()),
    );

    args.option(
        "",
        "image",
        "Builder image name",
        "<name:tag>",
        Occur::Optional,
        Some("zeus-builder:latest".to_owned()),
    );

    args.option(
        "",
        "imagearchive",
        "Builder image build archive",
        "<path>",
        Occur::Optional,
        Some("builder.tar.gz".to_owned()),
    );

    args.option(
        "",
        "dockerfile",
        "Builder image dockerfile",
        "<path>",
        Occur::Optional,
        Some("Dockerfile".to_owned()),
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
        match &(args.value_of::<String>("color").unwrap())[..] {
            "always" => log::ColorChoice::Always,
            "never" => log::ColorChoice::Never,
            _ => log::ColorChoice::Auto,
        },
    );

    let docker = match Docker::connect_with_local_defaults() {
        Ok(v) => {
            logger.v(log::Level::Success, "docker", "Successfully connected");
            v
        }
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
        docker: docker,

        verbose: args.value_of::<bool>("verbose").unwrap(),
        force: args.value_of::<bool>("force").unwrap(),

        builder_archive: args.value_of("imagearchive").unwrap(),
        builder_dockerfile: args.value_of("dockerfile").unwrap(),
        builder_image: args.value_of("image").unwrap(),

        packages: args.values_of("packages").unwrap_or(vec![]),
        build_dir: args.value_of("builddir").unwrap(),
    };

    if args.value_of::<bool>("sync").unwrap() {
        //operation::sync(logger, cfg)
    } else if args.value_of::<bool>("build-builder").unwrap() {
        operation::build::build(logger, cfg).await;
    } else {
        logger.v(
            log::Level::Error,
            config::PROGRAM_NAME,
            "No operation specified! See --help",
        );
        exit(1);
    }
}
