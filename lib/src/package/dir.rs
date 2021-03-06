use {
    bytecheck::CheckBytes,
    rkyv::{Archive, Deserialize, Serialize},
    std::{io, path::Path},
};

/// A `Build` stores all data of a software
#[derive(Archive, Deserialize, Serialize, PartialEq, Debug)]
#[archive_attr(derive(CheckBytes))]
pub struct Dir {
    /// The buffer for all data
    pub data: Vec<u8>,
}
impl Dir {
    /// Archives the directory at `path` and compresses it with a of compression level `compression`.
    /// # Errors
    /// Returns `std::io::Error` when opening the encoder `zstd::Encoder`,
    /// inserting the directory at `path` into the `tar::Builder` or
    /// finishing the `tar::Builder` failed.
    pub fn encode<P: AsRef<Path>>(path: P, compression: i32) -> io::Result<Self> {
        use {tar::Builder, zstd::Encoder};

        let path = path.as_ref();
        let buffer = Vec::new();

        // create archive builder with compression
        let mut builder = Builder::new(Encoder::new(buffer, compression)?);

        // add path to archive
        builder.append_dir_all(".", path)?;

        // finish streams and unpack data
        builder
            .into_inner()
            .and_then(Encoder::finish)
            .map(|data| Self { data })
    }
    /// Decodes a `Build` into a destination directory at path `dest`.
    /// # Errors
    /// Returns `std::io::Error` when opening the decoder `zstd::Decoder`
    /// or unpacking the archive fails.
    pub fn decode<D: AsRef<Path>>(&self, dest: D) -> io::Result<()> {
        use {tar::Archive, zstd::Decoder};

        let mut archive = Archive::new(Decoder::new(self.data.as_slice())?);
        archive.unpack(dest)?;

        archive.into_inner().finish();
        Ok(())
    }
}
