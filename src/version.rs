use crate::config;
use crate::error::ZeusError;

use bollard::Docker;

pub async fn version(docker: Docker) -> Result<(), ZeusError> {
    let defaults = config::Config::default();

    println!(
        "
     _oo     {} {}  -  docker client v{}
  >-(_  \\   
    / _/     Copyright lololol (C) 2022 1337 threadexio
   / /       
  / (        This program may be freely distributed under
 (   `-.     the terms of the GNU General Public License v3.0.
  `--.._)    
             
             Defaults:
               --archive    {}
               --dockerfile {}
               --image      {}
               --name       {}
               --builddir   {}
",
        config::PROGRAM_NAME,
        config::PROGRAM_VERSION,
        docker.client_version().to_string(),
        &defaults.builder.archive,
        &defaults.builder.dockerfile,
        &defaults.builder.image,
        &defaults.builder.name,
        &defaults.build_dir,
    );

    Ok(())
}
