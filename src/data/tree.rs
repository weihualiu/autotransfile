
/*
    多叉树记录目录，节点上使用vec记录文件
 */

use std::ptr::null;
use std::fs;
use std::path::Path;
use std::fs::{read_dir, ReadDir};
use std::ops::Add;
use std::io::Error;

#[derive(Debug)]
pub struct Node {
    files: Vec<String>,
    dirname: String,

    children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(dirname: String) -> Node {
        Node {
            files: vec![],
            dirname: dirname.to_string(),
            children: vec![]
        }
    }

    pub fn addFile(&mut self, fname: &str) {
        self.files.push(fname.to_string());
    }

    pub fn addChild(&mut self, node: Node) {
        self.children.push(Box::new(node));
    }
}

impl Node {
    /*
        遍历目录
     */
    pub fn read_dir(dirname: String, root_node: &mut Node) {
        if !dirname.is_empty() {
            let mut node = Node::new(dirname.clone());
            match fs::read_dir(Path::new(dirname.clone().as_str())) {
                Ok(paths) =>
                    for path in paths {
                        match path {
                            Ok(path1) =>
                               if path1.path().is_file() {
                                   node.addFile(&path1.file_name().to_str().unwrap());
                               }else if path1.path().is_dir() {
                                   let path_new = dirname.clone().add("/").add(path1.file_name().to_str().unwrap());
                                   // println!("{:?}", patht);
                                   Node::read_dir(path_new, &mut node);
                                },
                            Err(e) =>
                                println!("{:?}", e),
                        };
                    }
                Err(e) =>
                    println!("{:?}", e),
            };
            root_node.addChild(node);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_new() {
        let mut node = Node::new(String::from("/home"));
        node.addFile("test");
        node.addFile("test1");
        let mut node2 = Node::new(String::from("/embsewp"));
        node2.addFile("test2");
        node2.addFile("test22");
        node.addChild(node2);
        let mut node3 = Node::new(String::from("/embsewp2"));
        node3.addFile("test3");
        node3.addFile("test33");
        node.addChild(node3);
        println!("{:#?}", node);
    }

    #[test]
    fn node_read_dir() {
        let mut node = Node::new(String::from("/"));
        Node::read_dir(String::from("/Users/liuweihua/Documents"), &mut node);
        println!("{:#?}", node);
    }

}
