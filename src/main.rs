extern crate byteorder;
extern crate clap;
extern crate nbt;

use byteorder::{BigEndian, ReadBytesExt};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug)]
struct BlockEntity {
    id: String,
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Chunk {
    x: i32,
    z: i32,
    block_entities: Vec<BlockEntity>,
}

impl std::cmp::Ord for Chunk {
    fn cmp(&self, other: &Chunk) -> Ordering {
        match self.block_entities.len().cmp(&other.block_entities.len()) {
            Ordering::Equal => (self.x, self.z).cmp(&(other.x, other.z)),
            ordering => ordering,
        }
    }
}

impl std::cmp::PartialOrd for Chunk {
    fn partial_cmp(&self, other: &Chunk) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Chunk {
    fn eq(&self, other: &Chunk) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl std::cmp::Eq for Chunk {
}

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about(env!("CARGO_PKG_DESCRIPTION"))
                        .arg(clap::Arg::with_name("WORLD")
                            .help("Path of the world directory")
                            .required(true)
                            .index(1))
                        .arg(clap::Arg::with_name("DIMENSION")
                            .help("Dimension ID (-1 = The Nether; 0 = The Overworld; 1 = The End)")
                            .required(true)
                            .index(2))
                        .arg(clap::Arg::with_name("max-block-entities")
                            .long("max-block-entities")
                            .help("Maximum number of block entities to list for a chunk")
                            .takes_value(true)
                            .default_value("30"))
                        .arg(clap::Arg::with_name("max-chunks")
                            .long("max-chunks")
                            .help("Maximum chunks to list")
                            .takes_value(true)
                            .default_value("10"))
                        .after_help("WARNING: Negative dimension IDs like The Nether get interpreted as arguments. Use `--` to escape them. Example: block-entity-finder ~/.minecraft/saves/New\\ World -- -1")
                        .get_matches();

    let mut region_path = PathBuf::from(matches.value_of_os("WORLD").unwrap());
    let dimension = matches.value_of("DIMENSION").unwrap();
    if dimension != "0" {
        region_path.push(format!("DIM{}", dimension));
    }
    region_path.push("region");
    let mut chunks = BTreeSet::new();
    for entry in std::fs::read_dir(&region_path).expect("failed to open the region directory") {
        let mut region = std::fs::File::open(entry.expect("failed to read directory entry").path()).expect("failed to open region file");
        for i in 0..1024 {
            region.seek(SeekFrom::Start(i * 4)).expect("failed to seek to chunk index");
            let offset = region.read_u32::<BigEndian>().expect("failed to read chunk offset");

            let size = (offset & 0xFF) as u64 * 4 * 1024;
            let offset = (offset >> 8) as u64 * 4 * 1024;

            if size > 0 {
                region.seek(SeekFrom::Start(offset)).expect("failed to seek to chunk");
                region.read_u32::<BigEndian>().expect("failed to read chunk size");
                let compression = region.read_u8().expect("failed to read chunk compression scheme");
                let chunk = match compression {
                    1 => nbt::Blob::from_gzip(&mut region),
                    2 => nbt::Blob::from_zlib(&mut region),
                    n => panic!("Unrecognised compression scheme {}", n),
                }.expect("failed to read chunk");
                match chunk["Level"] {
                    nbt::Value::Compound(ref compound) => {
                        let (x, z) = match (compound.get("xPos"), compound.get("zPos")) {
                            (Some(&nbt::Value::Int(x)), Some(&nbt::Value::Int(z))) => (x, z),
                            _ => {
                                println!("chunk coordinates not `TAG_Int`s, ignoring chunk.");
                                continue
                            }
                        };
                        let block_entities = match compound.get("TileEntities") {
                            Some(&nbt::Value::List(ref block_entities)) => {
                                block_entities.iter()
                                    .filter_map(|entity| match *entity {
                                        nbt::Value::Compound(ref entity) => {
                                            Some(BlockEntity {
                                                id: match entity["id"] {
                                                    nbt::Value::String(ref id) => id.clone(),
                                                    _ => {
                                                        println!("block entity ID not a TAG_String, ignoring block entity.");
                                                        return None;
                                                    },
                                                },
                                                x: match entity["x"] {
                                                    nbt::Value::Int(x) => x,
                                                    _ => {
                                                        println!("block entity X coordinate not an TAG_Int, ignoring block entity.");
                                                        return None;
                                                    }
                                                },
                                                y: match entity["y"] {
                                                    nbt::Value::Int(y) => y,
                                                    _ => {
                                                        println!("block entity Y coordinate not an TAG_Int, ignoring block entity.");
                                                        return None;
                                                    }
                                                },
                                                z: match entity["z"] {
                                                    nbt::Value::Int(z) => z,
                                                    _ => {
                                                        println!("block entity Z coordinate not an TAG_Int, ignoring block entity.");
                                                        return None;
                                                    }
                                                },
                                            })
                                        },
                                        _ => {
                                            println!("block entity is not a TAG_Compound, ignoring.");
                                            None
                                        },
                                    })
                                    .collect::<Vec<_>>()
                            },
                            _ => {
                                println!("'Level' -> 'TileEntities' is not a list, ignoring.");
                                continue;
                            },
                        };
                        if block_entities.len() > 0 {
                            chunks.insert(Chunk {
                                x: x,
                                z: z,
                                block_entities: block_entities,
                            });
                        }
                    },
                    _ => println!("'Level' is not a TAG_Compound, ignoring."),
                }
            }
        }
    }

    let max_chunks = matches.value_of("max-chunks").unwrap().parse().expect("failed to parse --max-chunks");
    let max_block_entities = matches.value_of("max-block-entities").unwrap().parse().expect("failed to parse --max-block-entities");

    for chunk in chunks.iter().rev().take(max_chunks) {
        println!("Chunk ({}, {}): {} block entit{}", chunk.x, chunk.z, chunk.block_entities.len(), if chunk.block_entities.len() == 1 { "y" } else { "ies" });
        for (i, entity) in chunk.block_entities.iter().enumerate().take(max_block_entities) {
            println!("{:10}: {} at ({}, {}, {})", i + 1, entity.id, entity.x, entity.y, entity.z);
        }
        if chunk.block_entities.len() > max_block_entities {
            println!("            ...");
        }
    }
}
