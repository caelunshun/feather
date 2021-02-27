use crate::{io::VarInt, ProtocolVersion, Readable, Writeable};
use aes::Aes128;
use bytes::BytesMut;
use cfb8::{
    stream_cipher::{NewStreamCipher, StreamCipher},
    Cfb8,
};
use std::io::Cursor;

type AesCfb8 = Cfb8<Aes128>;
pub type CompressionThreshold = usize;

/// An encryption key for use with AES-CFB8.
pub type CryptKey = [u8; 16];

/// State to serialize and deserialize packets from a byte stream.
#[derive(Default)]
pub struct MinecraftCodec {
    /// If encryption is enabled, then this is the cryptor state.
    cryptor: Option<AesCfb8>,
    crypt_key: Option<CryptKey>,
    /// If compression is enabled, then this is the compression threshold.
    compression: Option<CompressionThreshold>,

    /// A buffer of received bytes.
    received_buf: BytesMut,
    /// Auxilary buffer for use with compression.
    staging_buf: Vec<u8>,
}

impl MinecraftCodec {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables encryption with the provided key.
    pub fn enable_encryption(&mut self, key: CryptKey) {
        // yes, Mojang uses the same nonce for each packet. don't ask me why.
        self.cryptor = Some(AesCfb8::new_var(&key, &key).expect("key size is invalid"));
        self.crypt_key = Some(key);
    }

    /// Enables compression with the provided compression threshold.
    pub fn enable_compression(&mut self, threshold: CompressionThreshold) {
        self.compression = Some(threshold);
    }

    /// Gets another `MinecraftCodec` with the same compression and encryption
    /// parameters.
    pub fn clone_with_settings(&self) -> MinecraftCodec {
        MinecraftCodec {
            cryptor: self
                .crypt_key
                .map(|key| AesCfb8::new_var(&key, &key).expect("key size is invalid")),
            crypt_key: self.crypt_key,
            compression: self.compression,
            received_buf: BytesMut::new(),
            staging_buf: Vec::new(),
        }
    }

    /// Writes a packet into the provided writer.
    pub fn encode(&mut self, packet: &impl Writeable, output: &mut Vec<u8>) {
        packet.write(&mut self.staging_buf, ProtocolVersion::V1_16_2);

        if let Some(threshold) = self.compression {
            self.encode_compressed(output, threshold);
        } else {
            self.encode_uncompressed(output);
        }

        if let Some(cryptor) = &mut self.cryptor {
            cryptor.encrypt(output);
        }

        self.staging_buf.clear();
    }

    fn encode_compressed(&mut self, _output: &mut Vec<u8>, _threshold: CompressionThreshold) {
        todo!()
    }

    fn encode_uncompressed(&mut self, output: &mut Vec<u8>) {
        // TODO: we should probably be able to determine the length without writing the packet,
        // which could remove an unnecessary copy.
        let length = self.staging_buf.len() as i32;
        VarInt(length).write(output, ProtocolVersion::V1_16_2);
        output.extend_from_slice(&self.staging_buf);
    }

    /// Accepts newly received bytes.
    pub fn accept(&mut self, bytes: &[u8]) {
        let start_index = self.received_buf.len();
        self.received_buf.extend(bytes);

        if let Some(cryptor) = &mut self.cryptor {
            // Decrypt the new data (but not the whole received buffer,
            // since old data was already decrypted)
            cryptor.decrypt(&mut self.received_buf[start_index..]);
        }
    }

    /// Gets the next packet that was received, if any.
    pub fn next_packet<T>(&mut self) -> anyhow::Result<Option<T>>
    where
        T: Readable,
    {
        let mut cursor = Cursor::new(&self.received_buf[..]);
        let packet = if let Ok(length) = VarInt::read(&mut cursor, ProtocolVersion::V1_16_2) {
            let length_field_length = cursor.position() as usize;

            if self.received_buf.len() - length_field_length >= length.0 as usize {
                cursor = Cursor::new(
                    &self.received_buf
                        [length_field_length..length_field_length + length.0 as usize],
                );
                let packet = T::read(&mut cursor, ProtocolVersion::V1_16_2)?;

                let bytes_read = cursor.position() as usize + length_field_length;
                self.received_buf = self.received_buf.split_off(bytes_read);

                Some(packet)
            } else {
                None
            }
        } else {
            None
        };

        Ok(packet)
    }
}
