
use std::result;

use nix::convert_ioctl_res;
use nix::ioctl_read;
use nix::ioctl_none;
use nix::ioctl_read_buf;
use nix::ioctl_write_int;
use nix::ioctl_write_ptr;
use nix::libc;
use nix::request_code_none;
use nix::request_code_read;
use nix::request_code_readwrite;
use nix::sys;

#[repr(C)]
pub struct hiddev_devinfo {
	pub bustype: u32,
	pub busnum: u32,
	pub devnum: u32,
	pub ifnum: u32,
	pub vendor: i16,
	pub product: i16,
	pub version: i16,
	pub num_applications: u32,
}

#[repr(C)]
pub struct hiddev_collection_info {
	pub index: u32,
	pub type_: u32,
	pub usage: u32,
	pub level: u32,
}

const HID_STRING_SIZE: usize = 256;
#[repr(C)]
pub struct hiddev_string_descriptor {
	pub index: i32,
	pub value: [u8; HID_STRING_SIZE],
}

#[repr(C)]
pub struct hiddev_report_info {
	pub report_type: u32,
	pub report_id: u32,
	pub num_fields: u32,
}

#[repr(C)]
pub struct hiddev_field_info {
	pub report_type: u32,
	pub report_id: u32,
	pub field_index: u32,
	pub maxusage: u32,
	pub flags: u32,
	pub physical: u32, // physical usage for this field
	pub logical: u32, // logical usage for this field
	pub application: u32, // application usage for this field
	pub logical_minimum: i32,
	pub logical_maximum: i32,
	pub physical_minimum: i32,
	pub physical_maximum: i32,
	pub unit_exponent: u32,
	pub unit: u32,
}

#[repr(C)]
pub struct hiddev_usage_ref {
	pub report_type: u32,
	pub report_id: u32,
	pub field_index: u32,
	pub usage_index: u32,
	pub usage_code: u32,
	pub value: i32,
}

const HID_MAX_MULTI_USAGES: usize = 1024;

#[repr(C)]
pub struct hiddev_usage_ref_multi {
	pub uref: hiddev_usage_ref,
	pub num_values: u32,
	pub values: [i32; HID_MAX_MULTI_USAGES],
}

pub const HID_REPORT_TYPE_INPUT: u32 = 1;
pub const HID_REPORT_ID_FIRST: u32 = 0x00000100;
pub const HID_REPORT_ID_NEXT: u32 = 0x00000200;

ioctl_read!(HIDIOCGVERSION, b'H', 0x01, i32);
pub unsafe fn HIDIOCAPPLICATION(fd: libc::c_int,
					data: i32)
					-> nix::Result<libc::c_int> {
	convert_ioctl_res!(libc::ioctl(fd, request_code_none!(b'H', 0x02) as sys::ioctl::ioctl_num_type, data))
}
ioctl_read!(HIDIOCGDEVINFO, b'H', 0x03, hiddev_devinfo);
ioctl_read!(HIDIOCGSTRING, b'H', 0x04, hiddev_string_descriptor);
ioctl_none!(HIDIOCINITREPORT, b'H', 0x05);
ioctl_read_buf!(HIDIOCGNAME, b'H', 0x06, u8);
ioctl_read!(HIDIOCGREPORT, b'H', 0x07, hiddev_report_info);
ioctl_write_ptr!(HIDIOCSREPORT, b'H', 0x08, hiddev_report_info);
pub unsafe fn HIDIOCGREPORTINFO(fd: libc::c_int,
	data: *mut hiddev_report_info)
	-> nix::Result<libc::c_int> {
	let res = libc::ioctl(fd, request_code_readwrite!(b'H', 0x09, ::std::mem::size_of::<hiddev_report_info>()) as sys::ioctl::ioctl_num_type, data);
	match res {
		-1 => nix::Result::Ok(-1),
		_ => convert_ioctl_res!(res),
	}
}
ioctl_read!(HIDIOCGFIELDINFO, b'H', 0x0A, hiddev_field_info);
ioctl_read!(HIDIOCGUSAGE, b'H', 0x0B, hiddev_usage_ref);
ioctl_write_ptr!(HIDIOCSUSAGE, b'H', 0x0C, hiddev_usage_ref);
ioctl_read!(HIDIOCGUCODE, b'H', 0x0D, hiddev_usage_ref);
ioctl_read!(HIDIOCGFLAG, b'H', 0x0E, i32);
ioctl_write_int!(HIDIOCSFLAG, b'H', 0x0F);
ioctl_read!(HIDIOCGCOLLECTIONINDEX, b'H', 0x10, hiddev_usage_ref);
ioctl_read!(HIDIOCGCOLLECTIONINFO, b'H', 0x11, hiddev_collection_info);
ioctl_read_buf!(HIDIOCGPHYS, b'H', 0x12, u8);

/* For writing/reading to multiple/consecutive usages */
ioctl_read!(HIDIOCGUSAGES, b'H', 0x13, hiddev_usage_ref_multi);
ioctl_write_ptr!(HIDIOCSUSAGES, b'H', 0x14, hiddev_usage_ref_multi);
