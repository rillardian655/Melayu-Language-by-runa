//! Backend Akselerasi Perkakasan GPU Dinamik (AMD ROCm / NVIDIA CUDA)
//!
//! Memuatkan `libamdhip64.so` + `librocblas.so` (ROCm) atau `libcudart.so` + `libcublas.so` (CUDA)
//! secara dinamik pada masa larian tanpa kebergantungan luar. Sekiranya tiada GPU disokong,
//! pengiraan akan jatuh balik (fallback) ke CPU selari multi-teras secara automatik.

#[cfg(target_os = "linux")]
use std::ffi::c_void;
#[cfg(target_os = "linux")]
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuKind {
    None,
    Rocm,
    Cuda,
}

impl GpuKind {
    pub fn nama(&self) -> &'static str {
        match self {
            GpuKind::Rocm => "AMD ROCm (HIP + rocBLAS)",
            GpuKind::Cuda => "NVIDIA CUDA (cuBLAS)",
            GpuKind::None => "Tiada (CPU Selari Multi-Teras)",
        }
    }
}

#[cfg(target_os = "linux")]
mod linux_ffi {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::{c_char, c_int};

    const RTLD_LAZY: c_int = 1;

    #[link(name = "dl")]
    extern "C" {
        fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }

    type FnMalloc = unsafe extern "C" fn(*mut *mut c_void, usize) -> c_int;
    type FnFree = unsafe extern "C" fn(*mut c_void) -> c_int;
    type FnMemcpy = unsafe extern "C" fn(*mut c_void, *const c_void, usize, c_int) -> c_int;
    type FnDeviceSync = unsafe extern "C" fn() -> c_int;
    type FnCreateHandle = unsafe extern "C" fn(*mut *mut c_void) -> c_int;
    type FnGemm<T> = unsafe extern "C" fn(
        *mut c_void,
        c_int,
        c_int,
        c_int,
        c_int,
        c_int,
        *const T,
        *const c_void,
        c_int,
        *const c_void,
        c_int,
        *const T,
        *mut c_void,
        c_int,
    ) -> c_int;

    pub struct GpuDriver {
        pub kind: GpuKind,
        pub malloc_fn: FnMalloc,
        pub free_fn: FnFree,
        pub memcpy_fn: FnMemcpy,
        pub sync_fn: FnDeviceSync,
        pub dgemm_fn: FnGemm<f64>,
        pub sgemm_fn: FnGemm<f32>,
        pub handle: std::sync::Mutex<usize>,
        pub op_none: c_int,
        pub memcpy_h2d: c_int,
        pub memcpy_d2h: c_int,
    }

    unsafe impl Send for GpuDriver {}
    unsafe impl Sync for GpuDriver {}

    unsafe fn load_lib(paths: &[&str]) -> *mut c_void {
        for p in paths {
            if let Ok(cstr) = CString::new(*p) {
                let h = dlopen(cstr.as_ptr(), RTLD_LAZY);
                if !h.is_null() {
                    return h;
                }
            }
        }
        std::ptr::null_mut()
    }

    unsafe fn load_cuda_pair() -> (*mut c_void, *mut c_void) {
        for (cudart, cublas) in [
            ("libcudart.so", "libcublas.so"),
            ("/usr/local/cuda/lib64/libcudart.so", "/usr/local/cuda/lib64/libcublas.so"),
            ("libcudart.so.13", "libcublas.so.13"),
            ("libcudart.so.12", "libcublas.so.12"),
            ("libcudart.so.11.0", "libcublas.so.11"),
        ] {
            let rt = load_lib(&[cudart]);
            let blas = load_lib(&[cublas]);
            if !rt.is_null() && !blas.is_null() {
                return (rt, blas);
            }
        }
        (std::ptr::null_mut(), std::ptr::null_mut())
    }

    unsafe fn load_sym(lib: *mut c_void, name: &str) -> *mut c_void {
        if let Ok(cstr) = CString::new(name) {
            dlsym(lib, cstr.as_ptr())
        } else {
            std::ptr::null_mut()
        }
    }

    fn rocm_arch_supported() -> bool {
        if std::env::var("HSA_OVERRIDE_GFX_VERSION").is_ok() {
            return true;
        }
        let Ok(entries) = std::fs::read_dir("/sys/class/kfd/kfd/topology/nodes") else {
            return true;
        };
        let mut found_gpu = false;
        for entry in entries.flatten() {
            let prop_path = entry.path().join("properties");
            if let Ok(content) = std::fs::read_to_string(prop_path) {
                for line in content.lines() {
                    if let Some(rest) = line.strip_prefix("gfx_target_version ") {
                        if let Ok(v) = rest.trim().parse::<u32>() {
                            if v > 0 {
                                found_gpu = true;
                                let arch = format!("gfx{}{:x}{:x}", v / 10000, (v / 100) % 100, v % 100);
                                let lazy_path = format!("/opt/rocm/lib/rocblas/library/TensileLibrary_lazy_{}.dat", arch);
                                let full_path = format!("/opt/rocm/lib/rocblas/library/TensileLibrary_{}.dat", arch);
                                if std::path::Path::new(&lazy_path).exists()
                                    || std::path::Path::new(&full_path).exists()
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        !found_gpu
    }

    pub fn init_driver() -> Option<GpuDriver> {
        if !super::gpu_dibenarkan() {
            return None;
        }
        unsafe {
            // 1. Cuba AMD ROCm (HIP + rocBLAS) jika seni bina GPU disokong oleh TensileLibrary
            if rocm_arch_supported() {
            let hip = load_lib(&[
                "/opt/rocm/lib/libamdhip64.so",
                "libamdhip64.so.7",
                "libamdhip64.so",
            ]);
            let rocblas = load_lib(&[
                "/opt/rocm/lib/librocblas.so",
                "librocblas.so.5",
                "librocblas.so",
            ]);

            if !hip.is_null() && !rocblas.is_null() {
                let malloc_p = load_sym(hip, "hipMalloc");
                let free_p = load_sym(hip, "hipFree");
                let memcpy_p = load_sym(hip, "hipMemcpy");
                let sync_p = load_sym(hip, "hipDeviceSynchronize");
                let create_p = load_sym(rocblas, "rocblas_create_handle");
                let dgemm_p = load_sym(rocblas, "rocblas_dgemm");
                let sgemm_p = load_sym(rocblas, "rocblas_sgemm");

                if !malloc_p.is_null()
                    && !free_p.is_null()
                    && !memcpy_p.is_null()
                    && !sync_p.is_null()
                    && !create_p.is_null()
                    && !dgemm_p.is_null()
                    && !sgemm_p.is_null()
                {
                    let create_fn: FnCreateHandle = std::mem::transmute(create_p);
                    let mut test_handle: *mut c_void = std::ptr::null_mut();
                    if create_fn(&mut test_handle) == 0 && !test_handle.is_null() {
                        let drv = GpuDriver {
                            kind: GpuKind::Rocm,
                            malloc_fn: std::mem::transmute(malloc_p),
                            free_fn: std::mem::transmute(free_p),
                            memcpy_fn: std::mem::transmute(memcpy_p),
                            sync_fn: std::mem::transmute(sync_p),
                            dgemm_fn: std::mem::transmute(dgemm_p),
                            sgemm_fn: std::mem::transmute(sgemm_p),
                            handle: std::sync::Mutex::new(test_handle as usize),
                            op_none: 111, // rocblas_operation_none
                            memcpy_h2d: 1, // hipMemcpyHostToDevice
                            memcpy_d2h: 2, // hipMemcpyDeviceToHost
                        };
                        if let Some(verified_drv) = self_test_with_timeout(drv) {
                            return Some(verified_drv);
                        }
                    }
                }
            }
            }

            // 2. Cuba NVIDIA CUDA (cudart + cuBLAS)
            let (cudart, cublas) = load_cuda_pair();
            if !cudart.is_null() && !cublas.is_null() {
                let malloc_p = load_sym(cudart, "cudaMalloc");
                let free_p = load_sym(cudart, "cudaFree");
                let memcpy_p = load_sym(cudart, "cudaMemcpy");
                let sync_p = load_sym(cudart, "cudaDeviceSynchronize");
                let create_p = load_sym(cublas, "cublasCreate_v2");
                let dgemm_p = load_sym(cublas, "cublasDgemm_v2");
                let sgemm_p = load_sym(cublas, "cublasSgemm_v2");

                if !malloc_p.is_null()
                    && !free_p.is_null()
                    && !memcpy_p.is_null()
                    && !sync_p.is_null()
                    && !create_p.is_null()
                    && !dgemm_p.is_null()
                    && !sgemm_p.is_null()
                {
                    let create_fn: FnCreateHandle = std::mem::transmute(create_p);
                    let mut test_handle: *mut c_void = std::ptr::null_mut();
                    if create_fn(&mut test_handle) == 0 && !test_handle.is_null() {
                        return Some(GpuDriver {
                            kind: GpuKind::Cuda,
                            malloc_fn: std::mem::transmute(malloc_p),
                            free_fn: std::mem::transmute(free_p),
                            memcpy_fn: std::mem::transmute(memcpy_p),
                            sync_fn: std::mem::transmute(sync_p),
                            dgemm_fn: std::mem::transmute(dgemm_p),
                            sgemm_fn: std::mem::transmute(sgemm_p),
                            handle: std::sync::Mutex::new(test_handle as usize),
                            op_none: 0, // CUBLAS_OP_N
                            memcpy_h2d: 1, // cudaMemcpyHostToDevice
                            memcpy_d2h: 2, // cudaMemcpyDeviceToHost
                        });
                    }
                }
            }

            None
        }
    }

    fn self_test_with_timeout(drv: GpuDriver) -> Option<GpuDriver> {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let a = [1.0, 2.0, 3.0, 4.0];
            let b = [1.0, 0.0, 0.0, 1.0];
            let ok = gemm_row_major(&drv, drv.dgemm_fn, 1.0f64, &a, &b, 2, 2, 2)
                .map(|res| res == [1.0, 2.0, 3.0, 4.0])
                .unwrap_or(false);
            let _ = tx.send(if ok { Some(drv) } else { None });
        });
        rx.recv_timeout(std::time::Duration::from_secs(2))
            .ok()
            .flatten()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn gemm_row_major<T: Copy + Default>(
        drv: &GpuDriver,
        gemm: FnGemm<T>,
        satu: T,
        a: &[T],
        b: &[T],
        m: usize,
        k: usize,
        n: usize,
    ) -> Option<Vec<T>> {
        let size_a = m.checked_mul(k)?.checked_mul(std::mem::size_of::<T>())?;
        let size_b = k.checked_mul(n)?.checked_mul(std::mem::size_of::<T>())?;
        let size_c = m.checked_mul(n)?.checked_mul(std::mem::size_of::<T>())?;

        unsafe {
            let guard = drv.handle.lock().ok()?;
            let handle = *guard as *mut c_void;
            if handle.is_null() {
                return None;
            }

            let mut d_a: *mut c_void = std::ptr::null_mut();
            let mut d_b: *mut c_void = std::ptr::null_mut();
            let mut d_c: *mut c_void = std::ptr::null_mut();

            if (drv.malloc_fn)(&mut d_a, size_a) != 0
                || (drv.malloc_fn)(&mut d_b, size_b) != 0
                || (drv.malloc_fn)(&mut d_c, size_c) != 0
            {
                if !d_a.is_null() { (drv.free_fn)(d_a); }
                if !d_b.is_null() { (drv.free_fn)(d_b); }
                if !d_c.is_null() { (drv.free_fn)(d_c); }
                return None;
            }

            let ok_copy = (drv.memcpy_fn)(d_a, a.as_ptr() as *const c_void, size_a, drv.memcpy_h2d) == 0
                && (drv.memcpy_fn)(d_b, b.as_ptr() as *const c_void, size_b, drv.memcpy_h2d) == 0;

            let mut out = vec![T::default(); m * n];
            let mut success = false;

            if ok_copy {
                let alpha = satu;
                let beta = T::default();
                // Row-major C(m x n) = A(m x k) * B(k x n) melalui column-major C^T = B^T * A^T
                let rc = gemm(
                    handle,
                    drv.op_none,
                    drv.op_none,
                    n as c_int,
                    m as c_int,
                    k as c_int,
                    &alpha,
                    d_b,
                    n as c_int,
                    d_a,
                    k as c_int,
                    &beta,
                    d_c,
                    n as c_int,
                );
                if rc == 0 && (drv.sync_fn)() == 0 {
                    if (drv.memcpy_fn)(out.as_mut_ptr() as *mut c_void, d_c, size_c, drv.memcpy_d2h) == 0 {
                        success = true;
                    }
                }
            }

            (drv.free_fn)(d_a);
            (drv.free_fn)(d_b);
            (drv.free_fn)(d_c);

            if success { Some(out) } else { None }
        }
    }
}

fn tetapan_gpu() -> String {
    std::env::var("MELAYU_GPU").unwrap_or_default().trim().to_lowercase()
}

pub fn gpu_dibenarkan() -> bool {
    !matches!(tetapan_gpu().as_str(), "0" | "tiada" | "mati")
}

pub fn ketepatan() -> &'static str {
    if tetapan_gpu() == "fp32" {
        "FP32"
    } else {
        "FP64"
    }
}

#[cfg(target_os = "linux")]
static GPU_DRIVER: OnceLock<Option<linux_ffi::GpuDriver>> = OnceLock::new();

#[cfg(target_os = "linux")]
pub fn gpu_kind() -> GpuKind {
    match GPU_DRIVER.get_or_init(linux_ffi::init_driver) {
        Some(drv) => drv.kind,
        None => GpuKind::None,
    }
}

#[cfg(not(target_os = "linux"))]
pub fn gpu_kind() -> GpuKind {
    GpuKind::None
}

#[cfg(target_os = "linux")]
pub fn gpu_matmul(a: &[f64], b: &[f64], m: usize, k: usize, n: usize) -> Option<Vec<f64>> {
    let drv = GPU_DRIVER.get_or_init(linux_ffi::init_driver).as_ref()?;
    if ketepatan() == "FP64" {
        return linux_ffi::gemm_row_major(drv, drv.dgemm_fn, 1.0, a, b, m, k, n);
    }
    let a: Vec<f32> = a.iter().map(|&x| x as f32).collect();
    let b: Vec<f32> = b.iter().map(|&x| x as f32).collect();
    let hasil = linux_ffi::gemm_row_major(drv, drv.sgemm_fn, 1.0, &a, &b, m, k, n)?;
    if hasil.iter().any(|x| !x.is_finite()) {
        return None;
    }
    Some(hasil.into_iter().map(f64::from).collect())
}

#[cfg(not(target_os = "linux"))]
pub fn gpu_matmul(_a: &[f64], _b: &[f64], _m: usize, _k: usize, _n: usize) -> Option<Vec<f64>> {
    None
}
