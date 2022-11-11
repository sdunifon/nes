use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

struct RomFile {
    file: File,
}

impl RomFile {
    // fn bytes(&mut self) -> Vec<u8> {
    //     let mut bytes = Vec::new();
    //     self.file.read_to_end(&mut bytes).unwrap();
    //     bytes
    // }
    fn bytes(&self){
        let mut buffer = [0; 16];
        let mut file = self.file.try_clone().unwrap();
        loop {
            let n = file.read(&mut buffer).unwrap();
            if n == 0 {
                break;
            }
            println!("{:?}", &buffer[..n]);
        }

    }
}



impl From<PathBuf> for RomFile{
    fn from(path: PathBuf) -> Self {
        let file = File::open(path).unwrap();
        Self { file }
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    #[test]
    fn from_path_buf() {
        let rom_file = RomFile::from(PathBuf::from("roms/nestest.nes"));
        rom_file.bytes();

    }
}