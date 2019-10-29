extern crate shiplift;
extern crate tokio;

use tokio::prelude::Future;

pub fn get_manifest(docker_tag: &String, logger: &slog::Logger) -> Result<(), String> {
    debug!(logger, "{}", "inspecting docker image to get seed label: ".to_string() + docker_tag);
    let docker = shiplift::Docker::new();
    let fut = docker.containers().get(&docker_tag).inspect()
        .map(|container| println!("{:#?}", container))
        .map_err(|e| eprintln!("Error: {}", e));
    error!(logger, )
    tokio::run(fut);
    Ok(())
}elo