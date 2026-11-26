use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn baca(akar: &Path, laluan: &Path, had: u64) -> io::Result<Vec<u8>> {
    let fail = pelantar::buka_baca(akar, laluan)?;
    sahkan_biasa(&fail)?;
    let mut isi = Vec::new();
    fail.take(had + 1).read_to_end(&mut isi)?;
    if isi.len() as u64 > had {
        return Err(ralat("Saiz fail melebihi had keselamatan 5 MB."));
    }
    Ok(isi)
}

pub fn tulis(akar: &Path, laluan: &Path, isi: &[u8]) -> io::Result<()> {
    let mut fail = pelantar::buka_tulis(akar, laluan)?;
    sahkan_biasa(&fail)?;
    fail.set_len(0)?;
    fail.write_all(isi)
}

pub fn padam(akar: &Path, laluan: &Path) -> io::Result<()> {
    pelantar::padam(akar, laluan)
}

pub fn wujud(akar: &Path, laluan: &Path) -> bool {
    pelantar::wujud(akar, laluan)
}

fn ralat(mesej: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, mesej)
}

fn sahkan_biasa(fail: &File) -> io::Result<()> {
    if fail.metadata()?.is_file() {
        Ok(())
    } else {
        Err(ralat("Laluan ini bukan fail biasa."))
    }
}

#[cfg(any(
    all(
        any(target_os = "linux", target_os = "android"),
        any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv64",
            target_arch = "loongarch64",
            target_arch = "powerpc64",
            target_arch = "s390x"
        )
    ),
    target_os = "macos",
    target_os = "ios"
))]
mod pelantar {
    use std::ffi::CString;
    use std::fs::File;
    use std::io;
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
    use std::os::raw::{c_char, c_int, c_uint};
    use std::os::unix::ffi::OsStrExt;
    use std::path::{Component, Path};

    extern "C" {
        fn openat(dirfd: c_int, laluan: *const c_char, bendera: c_int, ...) -> c_int;
        fn unlinkat(dirfd: c_int, laluan: *const c_char, bendera: c_int) -> c_int;
    }

    const O_WRONLY: c_int = 1;

    #[cfg(any(target_os = "linux", target_os = "android"))]
    mod bendera {
        use std::os::raw::c_int;
        pub const CREAT: c_int = 0o100;
        pub const CLOEXEC: c_int = 0x80000;
        pub const NONBLOCK: c_int = 0x800;
        #[cfg(any(target_arch = "arm", target_arch = "aarch64", target_arch = "powerpc64"))]
        pub const NOFOLLOW: c_int = 0x8000;
        #[cfg(any(target_arch = "arm", target_arch = "aarch64", target_arch = "powerpc64"))]
        pub const DIRECTORY: c_int = 0x4000;
        #[cfg(not(any(target_arch = "arm", target_arch = "aarch64", target_arch = "powerpc64")))]
        pub const NOFOLLOW: c_int = 0x20000;
        #[cfg(not(any(target_arch = "arm", target_arch = "aarch64", target_arch = "powerpc64")))]
        pub const DIRECTORY: c_int = 0x10000;
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    mod bendera {
        use std::os::raw::c_int;
        pub const CREAT: c_int = 0x200;
        pub const CLOEXEC: c_int = 0x100_0000;
        pub const NONBLOCK: c_int = 0x4;
        pub const NOFOLLOW: c_int = 0x100;
        pub const DIRECTORY: c_int = 0x10_0000;
    }

    const ASAS: c_int = bendera::NOFOLLOW | bendera::CLOEXEC | bendera::NONBLOCK;

    fn buka_di(dir: &OwnedFd, nama: &CString, pilihan: c_int) -> io::Result<OwnedFd> {
        loop {
            let fd = unsafe { openat(dir.as_raw_fd(), nama.as_ptr(), pilihan | ASAS, 0o666 as c_uint) };
            if fd >= 0 {
                return Ok(unsafe { OwnedFd::from_raw_fd(fd) });
            }
            let e = io::Error::last_os_error();
            if e.kind() != io::ErrorKind::Interrupted {
                return Err(e);
            }
        }
    }

    fn induk(akar: &Path, laluan: &Path) -> io::Result<(OwnedFd, CString)> {
        let dalam = laluan
            .strip_prefix(akar)
            .map_err(|_| super::ralat("Laluan berada di luar sandbox."))?;
        let mut nama = Vec::new();
        for bahagian in dalam.components() {
            match bahagian {
                Component::Normal(n) => nama.push(CString::new(n.as_bytes())?),
                _ => return Err(super::ralat("Laluan fail tidak sah.")),
            }
        }
        let akhir = nama.pop().ok_or_else(|| super::ralat("Laluan ini bukan fail biasa."))?;
        let mut dir = OwnedFd::from(File::open(akar)?);
        for n in &nama {
            dir = buka_di(&dir, n, bendera::DIRECTORY)?;
        }
        Ok((dir, akhir))
    }

    pub fn buka_baca(akar: &Path, laluan: &Path) -> io::Result<File> {
        let (dir, nama) = induk(akar, laluan)?;
        buka_di(&dir, &nama, 0).map(File::from)
    }

    pub fn buka_tulis(akar: &Path, laluan: &Path) -> io::Result<File> {
        let (dir, nama) = induk(akar, laluan)?;
        buka_di(&dir, &nama, O_WRONLY | bendera::CREAT).map(File::from)
    }

    pub fn padam(akar: &Path, laluan: &Path) -> io::Result<()> {
        let (dir, nama) = induk(akar, laluan)?;
        if unsafe { unlinkat(dir.as_raw_fd(), nama.as_ptr(), 0) } == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub fn wujud(akar: &Path, laluan: &Path) -> bool {
        if laluan == akar {
            return true;
        }
        match induk(akar, laluan).and_then(|(dir, nama)| buka_di(&dir, &nama, 0)) {
            Ok(_) => true,
            Err(e) => e.kind() == io::ErrorKind::PermissionDenied,
        }
    }
}

#[cfg(not(any(
    all(
        any(target_os = "linux", target_os = "android"),
        any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv64",
            target_arch = "loongarch64",
            target_arch = "powerpc64",
            target_arch = "s390x"
        )
    ),
    target_os = "macos",
    target_os = "ios"
)))]
mod pelantar {
    use std::fs::{self, File, OpenOptions};
    use std::io;
    use std::path::Path;

    pub fn buka_baca(_akar: &Path, laluan: &Path) -> io::Result<File> {
        File::open(laluan)
    }

    pub fn buka_tulis(_akar: &Path, laluan: &Path) -> io::Result<File> {
        OpenOptions::new().write(true).create(true).truncate(false).open(laluan)
    }

    pub fn padam(_akar: &Path, laluan: &Path) -> io::Result<()> {
        fs::remove_file(laluan)
    }

    pub fn wujud(_akar: &Path, laluan: &Path) -> bool {
        laluan.exists()
    }
}
