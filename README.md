<p align="center">
	<a href="https://github.com/Amber-PM/AmberMap">
		<img src=".github/readme/amberpm.png" width="128" height="128" alt="AmberMap Logo" title="AmberMap" />
	</a><br>
	<b>AmberMap: High-Performance Bedrock World Map Renderer written in Rust</b>
</p>

<p align="center">
	<a href="https://github.com/Amber-PM/AmberMap/releases/latest"><img alt="GitHub release (latest SemVer)" src="https://img.shields.io/github/v/release/Amber-PM/AmberMap?label=release&sort=semver"></a>
	<a href="https://discord.gg/k55gScjTs3"><img src="https://img.shields.io/badge/Discord-Chat-5865F2?logo=discord&logoColor=white" alt="Discord" /></a>
	<a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache--2.0%20%7C%20MIT-blue.svg" alt="License" /></a>
</p>

## Overview

**AmberMap** is a high-performance Minecraft: Bedrock Edition world map renderer and standalone web viewer written in pure Rust.

It parses Bedrock LevelDB databases directly, decompresses subchunk palettes, executes vertical voxel ray-marching with front-to-back alpha compositing, applies 2.5D topographic hillshading, and generates hierarchical WebP and PNG tile pyramids for web mapping libraries.

The web viewer (Leaflet.js SPA with real-time Minecraft coordinate projection) is embedded into the standalone executable, requiring no external web servers or runtime dependencies.

---

## Technical Highlights

- **Direct LevelDB Access:** Decodes Bedrock storage keys across all dimensions with native support for Deflate (Zlib Raw), Snappy, and Zstd compression.
- **Dense SubChunk Unpacking:** Bit-unpacker for SubChunk storage versions 8 and 9 (1, 2, 3, 4, 5, 6, 8, 16 bits per voxel) including multi-layer waterlogging.
- **Top-Down Ray Traverser:** Front-to-back alpha compositing ($Y_{\max} \to Y_{\min}$) with early ray termination on opaque surfaces.
- **2.5D Directional Hillshading:** Northwest ($315^\circ$) oblique slope shading based on local height derivatives ($\Delta h_x, \Delta h_z$).
- **Parallel Zoom Pyramids:** Multi-threaded Rayon pipeline with $2\times2$ alpha-weighted downsampling from $Z_{\max}$ to $Z_0$.
- **Embedded Web Client:** Single binary containing the complete Leaflet.js single-page application and HTTP server via `tiny_http` and `rust-embed`.

---

## Building

Requires Rust 1.75 or later.

```bash
git clone https://github.com/Amber-PM/AmberMap.git
cd AmberMap
cargo build --release
```

The compiled binary will be located at `target/release/ambermap` (or `ambermap.exe` on Windows).

---

## CLI Reference

### 1. Render World Map (`render`)

Renders a world into a hierarchical tile pyramid:

```bash
ambermap render <WORLD> [OPTIONS]
```

| Option | Description | Default |
|---|---|---|
| `WORLD` | Path to Bedrock world folder or `db` directory | *Required* |
| `-o, --output <DIR>` | Output directory for tile hierarchy | `./public/map` |
| `-d, --dimension <DIM>` | Dimension (`overworld`, `nether`, `the_end`) | `overworld` |
| `-z, --zoom-levels <N>` | Number of zoom levels ($Z_0 \dots Z_{N-1}$) | `5` |
| `-f, --format <FMT>` | Tile format (`webp` or `png`) | `webp` |
| `-t, --threads <N>` | Thread pool size (`0` = logical CPU count) | `0` |
| `--no-hillshading` | Disable 2.5D topographic shading | `false` |
| `-s, --serve` | Start embedded web server after rendering | `false` |
| `-p, --port <PORT>` | Web server port | `8080` |
| `-b, --bind <IP>` | Server bind address | `127.0.0.1` |
| `--open` | Open browser automatically | `false` |

```bash
# Example: Render and serve immediately
ambermap render "worlds/world" --serve --open
```

### 2. Standalone Web Server (`serve`)

Serves pre-rendered tiles with the embedded map viewer:

```bash
ambermap serve [OPTIONS]
```

| Option | Description | Default |
|---|---|---|
| `-d, --dir <DIR>` | Directory containing map tiles | `./public/map` |
| `-p, --port <PORT>` | HTTP listening port | `8080` |
| `-b, --bind <IP>` | Bind IP address | `127.0.0.1` |
| `-f, --format <FMT>` | Tile format (`webp` or `png`) | `webp` |
| `-z, --zoom-levels <N>` | Maximum zoom level | `5` |
| `--open` | Open browser automatically | `false` |

### 3. Inspect Chunk (`inspect`)

Outputs format versions, subchunk distribution, and block palettes for a specific chunk:

```bash
ambermap inspect <WORLD> -x <CHUNK_X> -z <CHUNK_Z> [-d <DIMENSION>]
```

### 4. Single Chunk Render (`render-chunk`)

Renders an individual $16 \times 16$ chunk directly to an image:

```bash
ambermap render-chunk <WORLD> -x <CHUNK_X> -z <CHUNK_Z> --output <FILE.png> [--scale 16]
```

---

## Library Usage

AmberMap can also be used as a Rust library:

```toml
[dependencies]
ambermap = { git = "https://github.com/Amber-PM/AmberMap" }
```

```rust
use ambermap::core::coordinates::ChunkPos;
use ambermap::leveldb::keys::Dimension;
use ambermap::leveldb::reader::WorldDb;
use ambermap::raster::{rasterize_chunk, ColorMap};

fn main() -> ambermap::Result<()> {
    let db = WorldDb::open("worlds/world")?;
    let chunk_data = db.get_chunk(ChunkPos::new(0, 0), Dimension::Overworld)?;

    let colormap = ColorMap::new();
    let raster = rasterize_chunk(&chunk_data, &colormap, true);

    println!("Decoded {} voxels", raster.pixels.len() / 4);
    Ok(())
}
```

---

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
