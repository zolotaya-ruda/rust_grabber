use std::fs::{self, ReadDir};

pub struct Reader{
    pub current_dir: String
}

impl Reader{
    pub fn new(current_dir: String) -> Reader {
        Reader {
            current_dir: current_dir.clone()
        }
    }

    pub fn change_disk(&mut self, disk: &str) -> Result<ReadDir, std::io::Error> {
        let current_dir = fs::read_dir(disk);
        
        match current_dir {
            Ok(dir) => {self.current_dir = disk.to_string(); return Ok(dir)},
            Err(E) => {return Err(E)} 
        }
        
    }

    pub fn ls(&mut self) -> Result<ReadDir, std::io::Error> {
        let current_dir = fs::read_dir(self.current_dir.clone());
        
        match current_dir {
            Ok(dir) => {return Ok(dir)},
            Err(E) => {return Err(E)} 
        }
        
    }

    pub fn cd(&mut self, path: &str) {
        let s = self.current_dir.clone();
        let s = s.replace(r"\", "/");

        match path {
            "./" => {
                    let current_path = s.split(r"/");
                    let mut split: Vec<&str> = current_path.collect();
                    split.pop();

                    if split.len() == 1 {
                        split.push("")
                    }

                    let resp = split.join(r"/");
                    self.current_dir = resp;
                },
            _ => {
                self.current_dir = format!("{}/{}", s, path);
                println!("{}", self.current_dir)
            }
        }
    }

}