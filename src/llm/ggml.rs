// Copyright 2023-present The Yumcoder Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
//
// Author: yumcoder (omid.jn@gmail.com)
//
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read};

// Constants representing file magic numbers
const FILE_MAGIC_GGML: u32 = 0x67676d6c;
const FILE_MAGIC_GGMF: u32 = 0x67676d66;
const FILE_MAGIC_GGJT: u32 = 0x67676a74;
const FILE_MAGIC_GGLA: u32 = 0x67676C61;
const FILE_MAGIC_GGUF_LE: u32 = 0x46554747;
const FILE_MAGIC_GGUF_BE: u32 = 0x47475546;

// Enum representing different container types

#[derive(Debug, PartialEq)]
enum ContainerType {
    GGML,
    GGMF { version: u32 },
    GGJT { version: u32 },
    GGLA { version: u32 },
    GGUF_LE,
    GGUF_BE,
}

// Enum representing different model types
#[derive(Debug, PartialEq)]
enum ModelType {
    Empty,
    Llama { hyperparameters: u32 },
}

// Struct representing a GGML model
#[derive(Debug)]
struct GGML {
    magic: u32,
    container: ContainerType,
    model: ModelType,
}

// Struct representing an empty model
struct EmptyModel;

// Struct representing a Llama model
struct LlamaModel {
    hyperparameters: u32,
}

// Function to read binary data and handle endianness
fn binary_read(reader: &mut dyn Read) -> Result<u32, io::Error> {
    Ok(reader.read_u32::<LittleEndian>()?)
}

// Function to decode GGML models
fn decode_ggml(reader: &mut dyn Read) -> Result<GGML, io::Error> {
    let magic = binary_read(reader)?;

    let container = match magic {
        FILE_MAGIC_GGML => ContainerType::GGML,
        FILE_MAGIC_GGMF => ContainerType::GGMF {
            version: binary_read(reader)?,
        },
        FILE_MAGIC_GGJT => ContainerType::GGJT {
            version: binary_read(reader)?,
        },
        FILE_MAGIC_GGLA => ContainerType::GGLA {
            version: binary_read(reader)?,
        },
        FILE_MAGIC_GGUF_LE => ContainerType::GGUF_LE,
        FILE_MAGIC_GGUF_BE => ContainerType::GGUF_BE,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid file magic",
            ))
        }
    };

    let model = match &container {
        ContainerType::GGML => ModelType::Empty,
        ContainerType::GGMF { version } => match version {
            1 => ModelType::Empty,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid version",
                ))
            }
        },
        ContainerType::GGJT { version } => match version {
            1 | 2 | 3 => ModelType::Llama {
                hyperparameters: binary_read(reader)?,
            },
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid version",
                ))
            }
        },
        ContainerType::GGLA { version } => match version {
            1 => ModelType::Empty,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid version",
                ))
            }
        },
        ContainerType::GGUF_BE => ModelType::Empty,
        ContainerType::GGUF_LE => ModelType::Empty,
    };

    Ok(GGML {
        magic,
        container,
        model,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to decode GGML from a byte vector
    fn decode_ggml_from_bytes(data: Vec<u8>) -> Result<GGML, io::Error> {
        let mut reader = std::io::Cursor::new(data);
        decode_ggml(&mut reader)
    }

    #[test]
    fn test_decode_ggml() {
        // Test decoding GGML
        let ggml = decode_ggml_from_bytes(vec![
            0x6c, 0x6d, 0x67, 0x67, // FILE_MAGIC_GGML
        ])
        .unwrap();
        assert_eq!(ggml.magic, FILE_MAGIC_GGML);
        assert_eq!(ggml.container, ContainerType::GGML);
        assert_eq!(ggml.model, ModelType::Empty);

        // Test decoding GGMF with valid version
        let ggml = decode_ggml_from_bytes(vec![
            0x66, 0x6d, 0x67, 0x67, // FILE_MAGIC_GGMF
            0x01, 0x00, 0x00, 0x00, // Version 1
        ])
        .unwrap();
        assert_eq!(ggml.magic, FILE_MAGIC_GGMF);
        assert_eq!(ggml.container, ContainerType::GGMF { version: 1 });
        assert_eq!(ggml.model, ModelType::Empty);

        // Test decoding GGMF with invalid version
        let result = decode_ggml_from_bytes(vec![
            0x66, 0x6d, 0x67, 0x67, // FILE_MAGIC_GGMF
            0x02, 0x00, 0x00, 0x00, // Invalid version
        ]);
        assert!(matches!(
            result.unwrap_err().kind(),
            io::ErrorKind::InvalidData
        ));

        // Test decoding GGJT with valid version and non-zero hyperparameters
        let ggml = decode_ggml_from_bytes(vec![
            0x74, 0x6a, 0x67, 0x67, // FILE_MAGIC_GGJT
            0x01, 0x00, 0x00, 0x00, // Version 1
            0x0A, 0x0B, 0x0C, 0x0D, // Non-zero Hyperparameters
        ])
        .unwrap();
        assert_eq!(ggml.magic, FILE_MAGIC_GGJT);
        assert_eq!(ggml.container, ContainerType::GGJT { version: 1 });
        assert_eq!(
            ggml.model,
            ModelType::Llama {
                hyperparameters: 0x0D0C0B0A
            }
        );

        // Test decoding GGJT with invalid version
        let result = decode_ggml_from_bytes(vec![
            0x74, 0x6a, 0x67, 0x67, // FILE_MAGIC_GGJT
            0x04, 0x00, 0x00, 0x00, // Invalid version
        ]);
        assert!(matches!(
            result.unwrap_err().kind(),
            io::ErrorKind::InvalidData
        ));
    }
}
