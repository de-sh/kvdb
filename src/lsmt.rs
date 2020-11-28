/// Log Structured Merge Tree
pub struct LSMT {}

impl LSMT {
    pub fn new() -> Self { Self {} }

    /// Writes a Vec of bytes into persistant storage, associated with key, another Vec of bytes.
    pub fn write(&self, key: Vec<&u8>, value: Vec<&u8>) -> Result<(), LSMTError> {
        todo!()
    }

    /// Tries obtaining the Vec of bytes associated with key from persistant storage. 
    pub fn get(&self, key: Vec<&u8>) -> Option<Vec<&u8>> {
        todo!()
    }
}

pub enum LSMTError {
    
}