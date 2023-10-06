use nix::{ioctl_none, ioctl_readwrite, ioctl_write_ptr};
use strum_macros::EnumIter;

const DRM_IOCTL_BASE: u32 = 'd' as u32;
const DRM_IOCTL_VERSION: u32 = 0x00;
const DRM_IOCTL_GET_CAP: u32 = 0x0c;
const DRM_IOCTL_SET_CLIENT_CAP: u32 = 0x0d;
const DRM_IOCTL_SET_MASTER: u32 = 0x1e;
const DRM_IOCTL_DROP_MASTER: u32 = 0x1f;
const DRM_IOCTL_ATTACH_MODE: u32 = 0xa8;
const DRM_IOCTL_DETACH_MODE: u32 = 0xa9;
const DRM_IOCTL_MODE_GETPLANERESOURCES: u32 = 0xb5;
const DRM_IOCTL_MODE_GETPLANE: u32 = 0xb6;

#[repr(C)]
#[derive(Debug, Default)]
pub struct drm_version {
    pub major: i32,
    pub minor: i32,
    pub patchlevel: i32,
    pub name_len: usize,
    pub name: u64,
    pub date_len: usize,
    pub date: u64,
    pub desc_len: usize,
    pub desc: u64,
}

ioctl_readwrite!(
    drm_ioctl_version,
    DRM_IOCTL_BASE,
    DRM_IOCTL_VERSION,
    drm_version
);

#[repr(C)]
#[derive(Debug, Default)]
pub struct drm_getcap {
    pub capability: u64,
    pub value: u64,
}

ioctl_readwrite!(
    drm_ioctl_get_cap,
    DRM_IOCTL_BASE,
    DRM_IOCTL_GET_CAP,
    drm_getcap
);

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
#[repr(u64)]
pub enum ClientCapability {
    Stereo3d = 1,
    UniversalPlanes,
    Atomic,
    AspectRatio,
    WritebackConnectors,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct drm_setclientcap {
    pub capability: u64,
    pub value: u64,
}

ioctl_write_ptr!(
    drm_ioctl_set_client_cap,
    DRM_IOCTL_BASE,
    DRM_IOCTL_SET_CLIENT_CAP,
    drm_setclientcap
);

ioctl_none!(drm_ioctl_set_master, DRM_IOCTL_BASE, DRM_IOCTL_SET_MASTER);

ioctl_none!(drm_ioctl_drop_master, DRM_IOCTL_BASE, DRM_IOCTL_DROP_MASTER);

ioctl_none!(drm_ioctl_attach_mode, DRM_IOCTL_BASE, DRM_IOCTL_ATTACH_MODE);

ioctl_none!(drm_ioctl_detach_mode, DRM_IOCTL_BASE, DRM_IOCTL_DETACH_MODE);

#[repr(C)]
#[derive(Debug, Default)]
pub struct drm_mode_get_plane_res {
    pub plane_id_ptr: u64,
    pub count_planes: u32,
}

ioctl_readwrite!(
    drm_ioctl_mode_getplaneresources,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETPLANERESOURCES,
    drm_mode_get_plane_res
);

#[repr(C)]
#[derive(Default, Debug)]
pub struct drm_mode_get_plane {
    pub plane_id: u32,
    pub crtc_id: u32,
    pub fb_id: u32,
    pub possible_crtcs: u32,
    pub gamma_size: u32,
    pub count_format_types: u32,
    pub format_type_ptr: u64,
}

ioctl_readwrite!(
    drm_ioctl_mode_getplane,
    DRM_IOCTL_BASE,
    DRM_IOCTL_MODE_GETPLANE,
    drm_mode_get_plane
);
