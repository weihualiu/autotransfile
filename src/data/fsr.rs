
/*
FSR结构定义描述：
--------------------------
1   | DM |
2   | Timestamp |
3   | Delete Directory Counts |
3.1 | Delete Directory |
4   | Delete Files Counts |
4.1 | Delete File |
---------------------------

 */

use super::dm::DM;

pub struct FSR {
    key: DM,
    timestamp: String,
    deleteDirectory: Vec<String>,
    deleteFile: Vec<String>,
}

impl FSR {
    pub fn new() -> FSR {
        FSR{
            key: DM::new(),
            timestamp: String::from(""),
            deleteDirectory: Vec::new(),
            deleteFile: Vec::new(),
        }
    }
}