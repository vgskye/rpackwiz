use digest::{Digest, DynDigest, Update, Reset, FixedOutput, OutputSizeUser, FixedOutputReset};

use crate::model::HashFormat;

#[derive(Clone)]
struct CurseforgeHasher(Vec<u8>);

impl Update for CurseforgeHasher {
    fn update(&mut self, data: &[u8]) {
        self.0.extend(data.iter().copied().filter(|&e| e != 9 && e != 10 && e != 13 && e != 32))
    }
}

impl Reset for CurseforgeHasher {
    fn reset(&mut self) {
        self.0 = Vec::new()
    }
}

impl OutputSizeUser for CurseforgeHasher {
    type OutputSize = digest::typenum::U4;
}

impl FixedOutput for CurseforgeHasher {
    fn finalize_into(self, out: &mut digest::Output<Self>) {
        *out = murmur2::murmur2(&self.0, 1).to_be_bytes().into();
    }
}

impl FixedOutputReset for CurseforgeHasher {
    fn finalize_into_reset(&mut self, out: &mut digest::Output<Self>) {
        *out = murmur2::murmur2(&self.0, 1).to_be_bytes().into();
        self.0 = Vec::new();
    }
}

pub fn hasher_for_format(format: HashFormat) -> Box<dyn DynDigest> {
    match format {
        HashFormat::Sha256 => Box::new(sha2::Sha256::new()),
        HashFormat::Sha512 => Box::new(sha2::Sha512::new()),
        HashFormat::Sha1 => Box::new(sha1::Sha1::new()),
        HashFormat::Md5 => Box::new(md5::Md5::new()),
        HashFormat::Curseforge => Box::new(CurseforgeHasher(Vec::new())),
    }
}