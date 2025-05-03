use std::collections::HashMap;

pub struct FileSystem {
    files: HashMap<String, Vec<u8>>,
    file_descriptors: HashMap<i32, (String, usize)>, // fd -> (filename, position)
    next_fd: i32,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            file_descriptors: HashMap::new(),
            next_fd: 3, // Start from 3 (0=stdin, 1=stdout, 2=stderr)
        }
    }
    
    pub fn add_file(&mut self, filename: &str, content: &[u8]) {
        self.files.insert(filename.to_string(), content.to_vec());
    }
    
    pub fn open(&mut self, filename: &str, _mode: i32) -> i32 {
        if let Some(_) = self.files.get(filename) {
            let fd = self.next_fd;
            self.next_fd += 1;
            self.file_descriptors.insert(fd, (filename.to_string(), 0));
            fd
        } else {
            println!("File not found: {}", filename);
            -1
        }
    }
    
    pub fn read(&mut self, fd: i32, buffer: &mut [u8], count: usize) -> i32 {
        if let Some((filename, position)) = self.file_descriptors.get_mut(&fd) {
            if let Some(content) = self.files.get(filename) {
                let remaining = content.len() - *position;
                let to_read = count.min(remaining);
                
                buffer[..to_read].copy_from_slice(&content[*position..*position + to_read]);
                *position += to_read;
                
                to_read as i32
            } else {
                -1
            }
        } else {
            -1
        }
    }
    
    pub fn close(&mut self, fd: i32) -> i32 {
        if self.file_descriptors.remove(&fd).is_some() {
            0
        } else {
            -1
        }
    }
}
