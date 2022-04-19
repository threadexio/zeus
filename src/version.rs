use crate::config;
use crate::error::ZeusError;

use bollard::Docker;

pub async fn version(docker: Docker) -> Result<(), ZeusError> {
    let defaults = config::Config::default();

    println!(
        "
     _oo     {program_name} {version}  -  docker client v{docker_version}
  >-(_  \\   
    / _/     Copyright lololol (C) 2022 1337 threadexio
   / /       
  / (        This program may be freely distributed under
 (   `-.     the terms of the GNU General Public License v3.0.
  `--.._)    
             {homepage}
             
             Defaults:
               --archive    {archive}
               --dockerfile {dockerfile}
               --image      {image}
               --name       {name}
               --builddir   {builddir}
",
        program_name = config::PROGRAM_NAME,
        version = config::PROGRAM_VERSION,
        docker_version = docker.client_version().to_string(),
        homepage = env!("CARGO_PKG_HOMEPAGE"),
        archive = &defaults.builder.archive,
        dockerfile = &defaults.builder.dockerfile,
        image = &defaults.builder.image,
        name = &defaults.builder.name,
        builddir = &defaults.build_dir,
    );

    Ok(())
}
