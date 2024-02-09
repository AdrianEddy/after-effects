use crate::*;

define_suite!(GPUDeviceSuite, PrSDKGPUDeviceSuite, kPrSDKGPUDeviceSuite, kPrSDKGPUDeviceSuiteVersion);

impl GPUDeviceSuite {
    pub fn new() -> Result<Self, Error> {
        crate::Suite::new()
    }
    pub fn get_device_count(&self) -> Result<usize, Error> {
        Ok(call_suite_fn_single!(self, GetDeviceCount -> u32)? as usize)
    }
    pub fn get_device_info(&self, device_index: u32) -> Result<pr_sys::PrGPUDeviceInfo, Error> {
        call_suite_fn_single!(self, GetDeviceInfo -> pr_sys::PrGPUDeviceInfo, pr_sys::kPrSDKGPUDeviceSuiteVersion, device_index)
    }
    /// Acquire/release exclusive access to inDeviceIndex. All calls below this point generally require access be held.
    /// For full GPU plugins (those that use a separate entry point for GPU rendering) exclusive access is always held.
    /// These calls do not need to be made in that case.
    /// For CUDA calls cuCtxPushCurrent/cuCtxPopCurrent on the current thread to manage the devices context.
    pub fn acquire_exclusive_device_access(&self, device_index: u32) -> Result<(), Error> {
        call_suite_fn!(self, AcquireExclusiveDeviceAccess, device_index)
    }
    pub fn release_exclusive_device_access(&self, device_index: u32) -> Result<(), Error> {
        call_suite_fn!(self, ReleaseExclusiveDeviceAccess, device_index)
    }
    /// All device memory must be allocated through this suite.
    /// Purge should be called only in emergency situations when working with GPU memory
    /// that cannot be allocated through this suite (eg OpenGL memory).
    /// Returned pointer value represents memory allocated through cuMemAlloc or clCreateBuffer.
    pub fn allocate_device_memory(&self, device_index: u32, size_in_bytes: usize) -> Result<*mut std::ffi::c_void, Error> {
        call_suite_fn_single!(self, AllocateDeviceMemory -> *mut std::ffi::c_void, device_index, size_in_bytes)
    }
    pub fn free_device_memory(&self, device_index: u32, ptr: *mut std::ffi::c_void) -> Result<(), Error> {
        call_suite_fn!(self, FreeDeviceMemory, device_index, ptr)
    }
    pub fn purge_device_memory(&self, device_index: u32, requested_bytes_to_purge: usize) -> Result<usize, Error> {
        call_suite_fn_single!(self, PurgeDeviceMemory -> usize, device_index, requested_bytes_to_purge)
    }
    /// All host (pinned) memory must be allocated through this suite.
    /// Purge should be called only in emergency situations when working with GPU memory
    /// that cannot be allocated through this suite (eg OpenGL memory).
    /// Returned pointer value represents memory allocated through cuMemHostAlloc or malloc.
    pub fn allocate_host_memory(&self, device_index: u32, size_in_bytes: usize) -> Result<*mut std::ffi::c_void, Error> {
        call_suite_fn_single!(self, AllocateHostMemory -> *mut std::ffi::c_void, device_index, size_in_bytes)
    }
    pub fn free_host_memory(&self, device_index: u32, ptr: *mut std::ffi::c_void) -> Result<(), Error> {
        call_suite_fn!(self, FreeHostMemory, device_index, ptr)
    }
    pub fn purge_host_memory(&self, device_index: u32, requested_bytes_to_purge: usize) -> Result<usize, Error> {
        call_suite_fn_single!(self, PurgeHostMemory -> usize, device_index, requested_bytes_to_purge)
    }
    /// Information on a GPU ppix. The following ppix functions may also be used:
    /// - PPixSuite::Dispose
    /// - PPixSuite::GetBounds
    /// - PPixSuite::GetRowBytes
    /// - PPixSuite::GetPixelAspectRatio
    /// - PPixSuite::GetPixelFormat
    /// - PPix2Suite::GetFieldOrder
    pub fn create_gpu_ppix(&self, device_index: u32, pixel_format: PixelFormat, width: i32, height: i32, par_numerator: i32, par_denominator: i32, field_type: pr_sys::prFieldType) -> Result<pr_sys::PPixHand, Error> {
        call_suite_fn_single!(self, CreateGPUPPix -> pr_sys::PPixHand, device_index, pixel_format.into(), width, height, par_numerator, par_denominator, field_type.into())
    }
    pub fn get_gpu_ppix_data(&self, ppix_handle: pr_sys::PPixHand) -> Result<*mut std::ffi::c_void, Error> {
        call_suite_fn_single!(self, GetGPUPPixData -> *mut std::ffi::c_void, ppix_handle)
    }
    pub fn get_gpu_ppix_device_index(&self, ppix_handle: pr_sys::PPixHand) -> Result<u32, Error> {
        call_suite_fn_single!(self, GetGPUPPixDeviceIndex -> u32, ppix_handle)
    }
    pub fn get_gpu_ppix_size(&self, ppix_handle: pr_sys::PPixHand) -> Result<usize, Error> {
        call_suite_fn_single!(self, GetGPUPPixSize -> usize, ppix_handle)
    }
}
