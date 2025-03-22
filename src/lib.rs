use std::ffi::OsStr;
use fuse::{
    FileType, FileAttr, Filesystem, Request, ReplyEntry,
    ReplyAttr, ReplyDirectory
};
use libc::ENOENT;
use log::debug;
use time::Timespec;


pub struct RustFS{
    root_attr: FileAttr,
}

impl RustFS {
     pub fn new() -> Self {
	 let now = time::now().to_timespec();	 

         let root_attr = FileAttr {
             ino: 1,
             size: 0,
             blocks: 0,
             atime: now,
             mtime: now,
             ctime: now,
             crtime: now,
             kind: FileType::Directory,
             perm: 0o755,
             nlink: 2,
             uid: 0,
             gid: 0,
             rdev: 0,
             flags: 0,
         };

         RustFS { root_attr }
     }
}

impl Filesystem for RustFS {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        debug!("getattr called for inode {}", ino);

        if ino == 1 {
            reply.attr(&Timespec::new(1,0), &self.root_attr);
        } else {
            reply.error(ENOENT);
        }
    }
    
   fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
       debug!("loolip called for parent {} and name {:?}", parent, name);

       if parent != 1 {
           reply.error(ENOENT);
           return;
       }

       reply.error(ENOENT);
   }

   fn readdir(
       &mut self,
       _req: &Request,
       ino: u64,
       _fn: u64,
       offset: i64,
       mut reply: ReplyDirectory,
   ) {
      debug!("readdir called for inode {} with offset {}", ino, offset);
      
      if ino != 1 {
          reply.error(ENOENT);
          return;
      }

      let entries = vec![
          (1, FileType::Directory, "."),
          (1, FileType::Directory, ".."),
      ];

      for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
           let (ino, kind, name) = entry;

           let full = reply.add(ino, (i + 1) as i64, kind, name);

           if full {
               break;
           }
        }
        
        reply.ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_rustfs() {
	let fs = RustFS::new();
	assert_eq!(fs.root_attr.ion, 1);
	assert_eq!(fs.root_attr.kind, FileType::Directory);
	assert_eq!(fs.root_attr.perm, 0o755);
    }
}
