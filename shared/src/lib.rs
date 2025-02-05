use serde::Deserialize;

#[derive(Deserialize)]
pub struct FileUpload {
    pub filedata: Vec<u8>,
    pub signature: Vec<u8>,
}
