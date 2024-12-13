use std::{
    fs::File,
    io::Error,
    mem::MaybeUninit,
    os::{
        fd::AsRawFd,
        raw::{c_uint, c_ulong},
    },
    path::PathBuf,
};

pub mod pps;
use pps::{pps_fdata, pps_kparams, pps_ktime, PPS_TIME_INVALID};

// These constants rely on macros which bindgen does not export from pps.h
const PPS_GETPARAMS: c_ulong = 0x800870a1;
const PPS_SETPARAMS: c_ulong = 0x400870a2;
const PPS_GETCAP: c_ulong = 0x800870a3;
const PPS_FETCH: c_ulong = 0xc00870a4;

pub struct PpsDevice(File);

impl PpsDevice {
    pub fn new(path: PathBuf) -> Result<PpsDevice, Error> {
        Ok(PpsDevice(File::open(path)?))
    }

    /// Perform ioctl request and check result for possible errors
    unsafe fn ioctl<T>(&self, request: c_ulong, value: &mut T) -> Result<(), Error> {
        let result = libc::ioctl(self.0.as_raw_fd(), request, value);
        if result != 0 {
            return Err(Error::last_os_error());
        }
        Ok(())
    }

    /// Perform ioctl request with uninitialized data
    unsafe fn ioctl_uninit<T>(&self, request: c_ulong) -> Result<T, Error> {
        let mut value: MaybeUninit<T> = MaybeUninit::uninit();
        self.ioctl(request, &mut value)?;
        Ok(unsafe { value.assume_init() })
    }

    pub fn get_params(&self) -> Result<pps_kparams, Error> {
        unsafe { self.ioctl_uninit(PPS_GETPARAMS) }
    }

    pub fn set_params(&self, params: &mut pps_kparams) -> Result<(), Error> {
        unsafe { self.ioctl(PPS_SETPARAMS, params) }
    }

    pub fn get_cap(&self) -> Result<c_uint, Error> {
        unsafe { self.ioctl_uninit(PPS_GETCAP) }
    }

    pub fn fetch(&self, timeout: Option<pps_ktime>) -> Result<pps_fdata, Error> {
        let timeout = timeout.unwrap_or(pps_ktime {
            sec: 0,
            nsec: 0,
            flags: PPS_TIME_INVALID,
        });

        let mut data = pps_fdata {
            info: Default::default(),
            timeout,
        };

        unsafe { self.ioctl(PPS_FETCH, &mut data)? };

        Ok(data)
    }
}
