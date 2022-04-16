use std::{io::Read, os::unix::net::UnixStream};

mod config;
mod log;

use log::Level;
use std::process::exit;

use std::process::Command;

fn main() {
    let mut logger = log::Logger::new(log::Stream::Stdout, log::ColorChoice::Auto);

    let socket_path = format!("/build/{}.sock", config::PROGRAM_NAME);
    let mut stream = match UnixStream::connect(&socket_path) {
        Ok(v) => v,
        Err(e) => {
            logger.v(
                Level::Error,
                "builder",
                format!("Cannot connect to socket: {}", e),
            );
            exit(1);
        }
    };

    let mut data = vec![0u8; 1024 * 8];
    let data_len: usize;
    match stream.read(&mut data[..]) {
        Ok(v) => {
            data_len = v;
        }
        Err(e) => {
            logger.v(
                Level::Error,
                "builder",
                format!("Cannot read data from socket: {}", e),
            );
            exit(1);
        }
    }

    // the &data[..data_len] is needed because serde_json doesn't stop parsing on a null byte
    let cfg: config::Config = match serde_json::from_slice(&data[..data_len]) {
        Ok(v) => v,
        Err(e) => {
            logger.v(
                Level::Error,
                "builder",
                format!("Cannot deserialize config: {}", e),
            );
            exit(1);
        }
    };

    for package in cfg.packages {
        let mut command = Command::new("/usr/bin/sudo");
        command.arg("-u");
        command.arg("builder");
        command.arg("/usr/local/bin/package_builder.sh");
        command.arg(&package);

        if cfg.upgrade {
            command.arg("Upgrade");
        } else {
            command.arg("Build");
        }

        command.args(&cfg.build_args);

        let mut child = match command.spawn() {
            Ok(v) => v,
            Err(e) => {
                logger.v(
                    Level::Error,
                    "builder",
                    format!("Cannot start package builder: {}", e),
                );
                exit(1);
            }
        };

        match child.wait() {
            Err(e) => {
                logger.v(
                    Level::Error,
                    "builder",
                    format!("Package builder error: {}", e),
                );
                exit(1);
            }
            Ok(v) => {
                if let Some(code) = v.code() {
                    if code != 0 {
                        logger.v(
                            Level::Warn,
                            "builder",
                            format!("Package build failed with code: {}", code),
                        );
                        continue;
                    } else {
                        logger.v(
                            Level::Success,
                            "builder",
                            format!("Package {} built successfully!", &package),
                        );
                    }
                }
            }
        }
    }
}
