# RustFS - A Filesystem Implementation in Rust

RustFS is a userspace filesystem implemented in Rust, based on the FUSE (Filesystem in Userspace) interface. This project aims to build a fully-functional, reliable, and high-performance filesystem from scratch, suitable for learning filesystem design and implementation principles.

## Current Progress

So far, we have completed the following:

- ✅ Basic project structure setup
- ✅ Dependency management configuration
- ✅ Minimal viable FUSE interface implementation
- ✅ Root directory basic functionality
  - Can view root directory attributes
  - Can list root directory (currently only `.` and `..` entries)

The current filesystem can be successfully mounted but doesn't support any file operations yet, providing only an empty root directory. This is the first step in building a filesystem and lays the foundation for further feature development.

## Usage

### Compilation

```bash
cargo build --release
```

### Running

```bash
# Create mount point
mkdir -p /tmp/rustfs

# Mount the filesystem in debug mode
RUST_LOG=debug ./target/release/rustfs /tmp/rustfs -d

# Check the mount in another terminal
ls -la /tmp/rustfs

# Unmount the filesystem
fusermount -u /tmp/rustfs  # Linux
umount /tmp/rustfs         # macOS
```

## Future Plans

The project development will be divided into the following phases:

### Phase 1: In-Memory Filesystem (In Progress)
- [ ] Implement file creation/deletion
- [ ] Support file read/write operations
- [ ] Add directory creation/deletion
- [ ] Implement basic file and directory permission management
- [ ] Enhance error handling
- [ ] Add unit and integration tests

### Phase 2: Persistent Storage
- [ ] Design disk format (superblock, inode table, data blocks, etc.)
- [ ] Implement block device abstraction layer
- [ ] Develop basic block allocator
- [ ] Implement filesystem formatting functionality
- [ ] Support filesystem mounting and unmounting
- [ ] Add basic caching mechanisms for performance

### Phase 3: Advanced Features
- [ ] Implement journaling/transaction mechanism for consistency
- [ ] Add data integrity verification
- [ ] Implement metadata backup and recovery
- [ ] Support symbolic links and hard links
- [ ] Optimize read/write performance
- [ ] Add asynchronous I/O support

### Phase 4: Production Readiness
- [ ] Comprehensive stress testing
- [ ] Fault recovery testing
- [ ] Develop filesystem checking tools
- [ ] Add backup/restore utilities
- [ ] Complete documentation and examples
- [ ] Performance benchmarking and optimization

### Optional Advanced Features
- [ ] Filesystem encryption
- [ ] Data compression
- [ ] Snapshots/version control
- [ ] Data deduplication
- [ ] Distributed support

## Tech Stack

- Rust programming language
- FUSE (Filesystem in Userspace)
- Time & Libc libraries
- Logging system (env_logger)
- Command-line argument parsing (clap)

## Contributing

Contributions are welcome. Please feel free to submit issue reports, feature requests, and pull requests. For major changes, please open an issue first to discuss what you would like to change.

## License

[Apache](LICENSE)

---

*Note: This is an educational project and not recommended for use in critical production environments unless thoroughly tested and validated.*
