/*
 *     _             _                __  __             
 *    / \   _ __ ___ | |__   ___ _ __ |  \/  | __ _ _ __  
 *   / _ \ | '_ ` _ \| '_ \ / _ \ '__|| |\/| |/ _` | '_ \ 
 *  / ___ \| | | | | | |_) |  __/ |   | |  | | (_| | |_) |
 * /_/   \_\_| |_| |_|_.__/ \___|_|   |_|  |_|\__,_| .__/ 
 *                                                 |_|    
 * 
 * AmberMap - High-Performance Bedrock World Map Renderer
 * https://github.com/Amber-PM/AmberMap
 *
 * Copyright (c) 2026 Amber-PM
 * Licensed under Apache-2.0 or MIT
 */

use crate::core::error::{AmberError, Result};
use crate::web::embedded::{get_embedded_asset, resolve_mime};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tiny_http::{Header, Response, Server, StatusCode};

pub struct ServerConfig {
    pub bind_addr: String,
    pub port: u16,
    pub tiles_dir: PathBuf,
    pub auto_open: bool,
    pub tile_format: String,
    pub max_zoom: u8,
}

pub fn start_server(config: ServerConfig) -> Result<()> {
    let addr = format!("{}:{}", config.bind_addr, config.port);
    let server = Server::http(&addr).map_err(|e| {
        AmberError::Io(std::io::Error::new(
            std::io::ErrorKind::AddrInUse,
            format!("failed to bind http server on {addr}: {e}"),
        ))
    })?;

    let url = format!("http://{}:{}", config.bind_addr, config.port);
    println!("[info] AmberMap Web Server listening on {}", url);
    println!("[info] Serving map tiles from {:?}", config.tiles_dir);
    println!("[info] Press Ctrl+C to stop the server");

    if config.auto_open {
        open_browser(&url);
    }

    for request in server.incoming_requests() {
        let raw_url = request.url().to_string();
        let path = raw_url.split('?').next().unwrap_or("/");

        if path == "/api/status" {
            let meta_path = config.tiles_dir.join("metadata.json");
            let json_body = if let Ok(meta) = std::fs::read_to_string(&meta_path) {
                meta
            } else {
                format!(
                    r#"{{"status":"ok","format":"{}","max_zoom":{},"center_x":0,"center_z":0}}"#,
                    config.tile_format, config.max_zoom
                )
            };
            let response = Response::from_string(json_body)
                .with_header(
                    Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap(),
                )
                .with_header(
                    Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap(),
                )
                .with_header(
                    Header::from_bytes(&b"Cache-Control"[..], &b"no-cache, no-store"[..]).unwrap(),
                );
            let _ = request.respond(response);
            continue;
        }

        if let Some(tile_subpath) = path.strip_prefix("/tiles/") {
            let file_path = config.tiles_dir.join(tile_subpath);
            if file_path.is_file() {
                if let Ok(mut file) = File::open(&file_path) {
                    let mut data = Vec::new();
                    if file.read_to_end(&mut data).is_ok() {
                        let mime = resolve_mime(tile_subpath);
                        let response = Response::from_data(data)
                            .with_header(
                                Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap(),
                            )
                            .with_header(
                                Header::from_bytes(
                                    &b"Cache-Control"[..],
                                    &b"public, max-age=3600"[..],
                                )
                                .unwrap(),
                            )
                            .with_header(
                                Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..])
                                    .unwrap(),
                            );
                        let _ = request.respond(response);
                        continue;
                    }
                }
            }

            let not_found = Response::from_string("Tile Not Found")
                .with_status_code(StatusCode(404))
                .with_header(
                    Header::from_bytes(&b"Content-Type"[..], &b"text/plain"[..]).unwrap(),
                );
            let _ = request.respond(not_found);
            continue;
        }

        if let Some((data, mime)) = get_embedded_asset(path) {
            let response = Response::from_data(data.into_owned())
                .with_header(Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap())
                .with_header(
                    Header::from_bytes(
                        &b"Cache-Control"[..],
                        &b"no-cache, no-store, must-revalidate"[..],
                    )
                    .unwrap(),
                );
            let _ = request.respond(response);
            continue;
        }

        if let Some((data, mime)) = get_embedded_asset("index.html") {
            let response = Response::from_data(data.into_owned())
                .with_header(Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap())
                .with_header(
                    Header::from_bytes(
                        &b"Cache-Control"[..],
                        &b"no-cache, no-store, must-revalidate"[..],
                    )
                    .unwrap(),
                );
            let _ = request.respond(response);
            continue;
        }

        let not_found = Response::from_string("Not Found").with_status_code(StatusCode(404));
        let _ = request.respond(not_found);
    }

    Ok(())
}

fn open_browser(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}
