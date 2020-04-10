extern crate anyhow;
extern crate nix;

use anyhow::Result;
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::io::{self, Write, Read, Seek, SeekFrom};

pub use nix::unistd;
pub use nix::sys::stat::Mode;


pub struct NamedPipe {
	file: File,
}

impl NamedPipe {
	pub fn new<P: AsRef<Path>>(path: P, mode: Mode) -> Result<NamedPipe> {
		unistd::mkfifo(path.as_ref(), mode)?;

		let file = OpenOptions::new()
			.create(true)
			.read(true)
			.write(true)
			.open(path.as_ref())?;

		Ok(NamedPipe {
			file,
		})
	}
}

impl Read for NamedPipe {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.file.read(buf)
	}
}

impl Write for NamedPipe {
	fn flush(&mut self) -> io::Result<()> {
		self.file.flush()
	}

	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.file.write(buf)
	}
}

impl Seek for NamedPipe {
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		self.file.seek(pos)
	}
}