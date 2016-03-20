use foreign;

use core::mem;

use rcstring::CString;

pub struct File {
	fil: foreign::FIL,
}

impl File {
	pub fn open(path: &CString, mode: u8) -> Result<File, foreign::FRESULT> {
		let mut fil: foreign::FIL = unsafe { mem::zeroed() };
		let res = unsafe {
			let string = match foreign::make_tchar_string(path) {
				Some(s) => s,
				None => return Err(foreign::FRESULT::FR_INVALID_NAME)
			};
			foreign::f_open(&mut fil, string, mode)
		};
		match res {
			foreign::FRESULT::FR_OK => {
				Ok(File {
					fil: fil
				})
			},
			_ => Err(res),
		}
	}

	fn ffi_fp(&mut self) -> *mut foreign::FIL {
		&mut self.fil
	}

	pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, foreign::FRESULT> {
		let mut bytes_read: foreign::UINT = 0;
		let res = unsafe { foreign::f_read(self.ffi_fp(), buf.as_mut_ptr() as foreign::voidp_mut, buf.len() as foreign::UINT, &mut bytes_read) };
		match res {
			foreign::FRESULT::FR_OK => Ok(bytes_read as usize),
			_ => Err(res)
		}
	}

	pub fn write(&mut self, buf: &[u8]) -> Result<usize, foreign::FRESULT> {
		let mut bytes_written: foreign::UINT = 0;
		let res = unsafe { foreign::f_write(self.ffi_fp(), buf.as_ptr() as foreign::voidp, buf.len() as foreign::UINT, &mut bytes_written) };
		match res {
			foreign::FRESULT::FR_OK => Ok(bytes_written as usize),
			_ => Err(res)
		}
	}

	pub fn lseek(&mut self, offset: u32) -> Result<(), foreign::FRESULT> {
		let res = unsafe { foreign::f_lseek(self.ffi_fp(), offset as foreign::DWORD) };
		match res {
			foreign::FRESULT::FR_OK => Ok(()),
			_ => Err(res)
		}
	}

	pub fn tell(&self) -> Result<u32, foreign::FRESULT> {
		Ok(self.fil.fptr as u32)
	}

	pub fn truncate(&mut self) -> Result<(), foreign::FRESULT> {
		let res = unsafe { foreign::f_truncate(self.ffi_fp()) };
		match res {
			foreign::FRESULT::FR_OK => Ok(()),
			_ => Err(res)
		}
	}

	pub fn sync(&mut self) -> Result<(), foreign::FRESULT> {
		let res = unsafe { foreign::f_sync(self.ffi_fp()) };
		match res {
			foreign::FRESULT::FR_OK => Ok(()),
			_ => Err(res)
		}
	}

	pub fn size(&self) -> Result<u32, foreign::FRESULT> {
		Ok(self.fil.fsize as u32)
	}
}

impl Drop for File {
	fn drop(&mut self) {
		unsafe { foreign::f_close(self.ffi_fp()); }
	}
}
