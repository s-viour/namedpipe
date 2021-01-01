extern crate anyhow;
extern crate nix;

use anyhow::Result;
use std::fs::{OpenOptions, File};
use std::path::Path;
use std::io::{self, Write, Read, Seek, SeekFrom};

use nix::unistd;
use nix::sys::stat::Mode;
use nix::errno::Errno;


pub struct NamedPipe {
	file: File,
}

impl NamedPipe {
	pub fn new<P: AsRef<Path>>(path: P, mode: Mode) -> Result<NamedPipe> {
		/* grab the result of mkfifo */
		let mkfifo_result = unistd::mkfifo(path.as_ref(), mode);

		/* we need to ignore any EEXIST errors that occur,
		 * because they're okay in this context
		 * so first, we check if the result is an error */
		if mkfifo_result.is_err() {
			/* if it is, then we destructure the error enum */
			match mkfifo_result.err().unwrap() {
				/* in the case that it's of variant Sys(errno) */
				nix::Error::Sys(e) => {
					/* check and see if the errno is EEXIST */
					if e != Errno::EEXIST {
						/* if it ISN'T EEXIST, then actually produce an error */
						mkfifo_result?
					}
				}
				/* in any other variant, it's an actual error, so produce one */
				_ => mkfifo_result?
			}
		}

		let file = OpenOptions::new()
			.create(true)
			.read(true)
			.write(true)
			.open(path.as_ref())?;

		Ok(NamedPipe {
			file,
		})
	}

	pub fn file(&mut self) -> &mut File {
		&mut self.file
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
