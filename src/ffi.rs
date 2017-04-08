#![allow(dead_code, non_camel_case_types, unused_variables, non_snake_case)]
use std::os::raw::{c_uint, c_char, c_int, c_longlong, c_void, c_uchar};

pub const VAPOURSYNTH_API_MAJOR: c_uint = 3;
pub const VAPOURSYNTH_API_MINOR: c_uint = 5;
pub const VAPOURSYNTH_API_VERSION: c_uint = (VAPOURSYNTH_API_MAJOR << 16) | (VAPOURSYNTH_API_MINOR);

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct VSFrameRef([u8; 0]);
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct VSNodeRef([u8; 0]);
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct VSCore([u8; 0]);
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct VSPlugin([u8; 0]);
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct VSNode([u8; 0]);
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct VSFuncRef([u8; 0]);
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct VSMap([u8; 0]);

pub enum VSFrameRef {}
pub enum VSNodeRef {}
pub enum VSCore {}
pub enum VSPlugin {}
pub enum VSNode {}
pub enum VSFuncRef {}
pub enum VSMap {}
pub enum VSFrameContext {}

// Constants
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VSColorFamily {
    cmGray = 1000000,
    cmRGB = 2000000,
    cmYUV = 3000000,
    cmYCoCg = 4000000,
    cmCompat = 9000000,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VSSampleType {
    stInteger = 0,
    stFloat = 1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VSPresetFormat {
    pfNone = 0,
    pfGray8 = 1000010,
    pfGray16 = 1000011,
    pfGrayH = 1000012,
    pfGrayS = 1000013,
    pfYUV420P8 = 3000010,
    pfYUV422P8 = 3000011,
    pfYUV444P8 = 3000012,
    pfYUV410P8 = 3000013,
    pfYUV411P8 = 3000014,
    pfYUV440P8 = 3000015,
    pfYUV420P9 = 3000016,
    pfYUV422P9 = 3000017,
    pfYUV444P9 = 3000018,
    pfYUV420P10 = 3000019,
    pfYUV422P10 = 3000020,
    pfYUV444P10 = 3000021,
    pfYUV420P16 = 3000022,
    pfYUV422P16 = 3000023,
    pfYUV444P16 = 3000024,
    pfYUV444PH = 3000025,
    pfYUV444PS = 3000026,
    pfYUV420P12 = 3000027,
    pfYUV422P12 = 3000028,
    pfYUV444P12 = 3000029,
    pfYUV420P14 = 3000030,
    pfYUV422P14 = 3000031,
    pfYUV444P14 = 3000032,
    pfRGB24 = 2000010,
    pfRGB27 = 2000011,
    pfRGB30 = 2000012,
    pfRGB48 = 2000013,
    pfRGBH = 2000014,
    pfRGBS = 2000015,
    pfCompatBGR32 = 9000010,
    pfCompatYUY2 = 9000011,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VSFilterMode {
    fmParallel = 100,
    fmParallelRequests = 200,
    fmUnordered = 300,
    fmSerial = 400,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VSNodeFlags {
    nfNoCache = 1,
    nfIsCache = 2,
    nfMakeLinear = 4,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VSPropTypes {
    ptUnset = 117,
    ptInt = 105,
    ptFloat = 102,
    ptData = 115,
    ptNode = 99,
    ptFrame = 118,
    ptFunction = 109,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VSGetPropErrors {
    peUnset = 1,
    peType = 2,
    peIndex = 4,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VSPropAppendMode {
    paReplace = 0,
    paAppend = 1,
    paTouch = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VSActivationReason {
    arInitial = 0,
    arFrameReady = 1,
    arAllFramesReady = 2,
    arError = -1,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VSMessageType {
    mtDebug = 0,
    mtWarning = 1,
    mtCritical = 2,
    mtFatal = 3,
}

// Interface Types
#[repr(C)]
#[derive(Debug, Copy)]
pub struct VSAPI {
    pub createCore: extern "C" fn(threads: c_int) -> *mut VSCore,
    pub freeCore: extern "C" fn(core: *mut VSCore),
    pub getCoreInfo: extern "C" fn(core: *mut VSCore) -> *const VSCoreInfo,
    pub cloneFrameRef: extern "C" fn(f: *const VSFrameRef) -> *const VSFrameRef,
    pub cloneNodeRef: extern "C" fn(node: *mut VSNodeRef) -> *mut VSNodeRef,
    pub cloneFuncRef: extern "C" fn(f: *mut VSFuncRef) -> *mut VSFuncRef,
    pub freeFrame: extern "C" fn(f: *const VSFrameRef),
    pub freeNode: extern "C" fn(node: *mut VSNodeRef),
    pub freeFunc: extern "C" fn(f: *mut VSFuncRef),
    pub newVideoFrame: extern "C" fn(format: *const VSFormat,
                                     width: c_int,
                                     height: c_int,
                                     propSrc: *const VSFrameRef,
                                     core: *mut VSCore)
                                     -> *mut VSFrameRef,
    pub copyFrame: extern "C" fn(f: *const VSFrameRef, core: *mut VSCore) -> *mut VSFrameRef,
    pub copyFrameProps: extern "C" fn(src: *const VSFrameRef,
                                      dst: *mut VSFrameRef,
                                      core: *mut VSCore),
    pub registerFunction: extern "C" fn(name: *const c_char,
                                        args: *const c_char,
                                        argsFunc: VSPublicFunction,
                                        functionData: *mut c_void,
                                        plugin: *mut VSPlugin),
    pub getPluginById: extern "C" fn(identifier: *const c_char, core: *mut VSCore) -> *mut VSPlugin,
    pub getPluginByNs: extern "C" fn(ns: *const c_char, core: *mut VSCore) -> *mut VSPlugin,
    pub getPlugins: extern "C" fn(core: *mut VSCore) -> *mut VSMap,
    pub getFunctions: extern "C" fn(plugin: *mut VSPlugin) -> *mut VSMap,
    pub createFilter: extern "C" fn(in_: *const VSMap,
                                    out: *mut VSMap,
                                    name: *const c_char,
                                    init: VSFilterInit,
                                    getFrame: VSFilterGetFrame,
                                    free: VSFilterFree,
                                    filterMode: c_int,
                                    flags: c_int,
                                    instanceData: *mut c_void,
                                    core: *mut VSCore),
    pub setError: extern "C" fn(map: *mut VSMap, errorMessage: *const c_char),
    pub getError: extern "C" fn(map: *const VSMap) -> *const c_char,
    pub setFilterError: extern "C" fn(errorMessage: *const c_char, frameCtx: *mut VSFrameContext),
    pub invoke: extern "C" fn(plugin: *mut VSPlugin,
                              name: *const c_char,
                              args: *const VSMap)
                              -> *mut VSMap,
    pub getFormatPreset: extern "C" fn(id: c_int, core: *mut VSCore) -> *const VSFormat,
    pub registerFormat: extern "C" fn(colorFamily: c_int,
                                      sampleType: c_int,
                                      bitsPerSample: c_int,
                                      subSamplingW: c_int,
                                      subSamplingH: c_int,
                                      core: *mut VSCore)
                                      -> *const VSFormat,
    pub getFrame: extern "C" fn(n: c_int,
                                node: *mut VSNodeRef,
                                errorMsg: *mut c_char,
                                bufSize: c_int)
                                -> *const VSFrameRef,
    pub getFrameAsync: extern "C" fn(n: c_int,
                                     node: *mut VSNodeRef,
                                     callback: VSFrameDoneCallback,
                                     userData: *mut c_void),
    pub getFrameFilter: extern "C" fn(n: c_int,
                                      node: *mut VSNodeRef,
                                      frameCtx: *mut VSFrameContext)
                                      -> *const VSFrameRef,
    pub requestFrameFilter: extern "C" fn(n: c_int,
                                          node: *mut VSNodeRef,
                                          frameCtx: *mut VSFrameContext),
    pub queryCompletedFrame: extern "C" fn(node: *mut *mut VSNodeRef,
                                           n: *mut c_int,
                                           frameCtx: *mut VSFrameContext),
    pub releaseFrameEarly: extern "C" fn(node: *mut VSNodeRef,
                                         n: c_int,
                                         frameCtx: *mut VSFrameContext),
    pub getStride: extern "C" fn(f: *const VSFrameRef, plane: c_int) -> c_int,
    pub getReadPtr: extern "C" fn(f: *const VSFrameRef, plane: c_int) -> *const c_uchar,
    pub getWritePtr: extern "C" fn(f: *mut VSFrameRef, plane: c_int) -> *mut c_uchar,
    pub createFunc: extern "C" fn(func: VSPublicFunction,
                                  userData: *mut c_void,
                                  free: VSFreeFuncData,
                                  core: *mut VSCore,
                                  vsapi: *const VSAPI)
                                  -> *mut VSFuncRef,
    pub callFunc: extern "C" fn(func: *mut VSFuncRef,
                                in_: *const VSMap,
                                out: *mut VSMap,
                                core: *mut VSCore,
                                vsapi: *const VSAPI),
    pub createMap: extern "C" fn() -> *mut VSMap,
    pub freeMap: extern "C" fn(map: *mut VSMap),
    pub clearMap: extern "C" fn(map: *mut VSMap),
    pub getVideoInfo: extern "C" fn(node: *mut VSNodeRef) -> *const VSVideoInfo,
    pub setVideoInfo: extern "C" fn(vi: *const VSVideoInfo, numOutputs: c_int, node: *mut VSNode),
    pub getFrameFormat: extern "C" fn(f: *const VSFrameRef) -> *const VSFormat,
    pub getFrameWidth: extern "C" fn(f: *const VSFrameRef, plane: c_int) -> c_int,
    pub getFrameHeight: extern "C" fn(f: *const VSFrameRef, plane: c_int) -> c_int,
    pub getFramePropsRO: extern "C" fn(f: *const VSFrameRef) -> *const VSMap,
    pub getFramePropsRW: extern "C" fn(f: *mut VSFrameRef) -> *mut VSMap,
    pub propNumKeys: extern "C" fn(map: *const VSMap) -> c_int,
    pub propGetKey: extern "C" fn(map: *const VSMap, index: c_int) -> *const c_char,
    pub propNumElements: extern "C" fn(map: *const VSMap, key: *const c_char) -> c_int,
    pub propGetType: extern "C" fn(map: *const VSMap, key: *const c_char) -> c_char,
    pub propGetInt: extern "C" fn(map: *const VSMap,
                                  key: *const c_char,
                                  index: c_int,
                                  error: *mut c_int)
                                  -> c_longlong,
    pub propGetFloat: extern "C" fn(map: *const VSMap,
                                    key: *const c_char,
                                    index: c_int,
                                    error: *mut c_int)
                                    -> f64,
    pub propGetData: extern "C" fn(map: *const VSMap,
                                   key: *const c_char,
                                   index: c_int,
                                   error: *mut c_int)
                                   -> *const c_char,
    pub propGetDataSize: extern "C" fn(map: *const VSMap,
                                       key: *const c_char,
                                       index: c_int,
                                       error: *mut c_int)
                                       -> c_int,
    pub propGetNode: extern "C" fn(map: *const VSMap,
                                   key: *const c_char,
                                   index: c_int,
                                   error: *mut c_int)
                                   -> *mut VSNodeRef,
    pub propGetFrame: extern "C" fn(map: *const VSMap,
                                    key: *const c_char,
                                    index: c_int,
                                    error: *mut c_int)
                                    -> *const VSFrameRef,
    pub propGetFunc: extern "C" fn(map: *const VSMap,
                                   key: *const c_char,
                                   index: c_int,
                                   error: *mut c_int)
                                   -> *mut VSFuncRef,
    pub propDeleteKey: extern "C" fn(map: *mut VSMap, key: *const c_char) -> c_int,
    pub propSetInt: extern "C" fn(map: *mut VSMap, key: *const c_char, i: i64, append: c_int)
                                  -> c_int,
    pub propSetFloat: extern "C" fn(map: *mut VSMap, key: *const c_char, d: f64, append: c_int)
                                    -> c_int,
    pub propSetData: extern "C" fn(map: *mut VSMap,
                                   key: *const c_char,
                                   data: *const c_char,
                                   size: c_int,
                                   append: c_int)
                                   -> c_int,
    pub propSetNode: extern "C" fn(map: *mut VSMap,
                                   key: *const c_char,
                                   node: *mut VSNodeRef,
                                   append: c_int)
                                   -> c_int,
    pub propSetFrame: extern "C" fn(map: *mut VSMap,
                                    key: *const c_char,
                                    f: *const VSFrameRef,
                                    append: c_int)
                                    -> c_int,
    pub propSetFunc: extern "C" fn(map: *mut VSMap,
                                   key: *const c_char,
                                   func: *mut VSFuncRef,
                                   append: c_int)
                                   -> c_int,
    pub setMaxCacheSize: extern "C" fn(bytes: i64, core: *mut VSCore) -> c_longlong,
    pub getOutputIndex: extern "C" fn(frameCtx: *mut VSFrameContext) -> c_int,
    pub newVideoFrame2: extern "C" fn(format: *const VSFormat,
                                      width: c_int,
                                      height: c_int,
                                      planeSrc: *mut *const VSFrameRef,
                                      planes: *const c_int,
                                      propSrc: *const VSFrameRef,
                                      core: *mut VSCore)
                                      -> *mut VSFrameRef,
    pub setMessageHandler: extern "C" fn(handler: VSMessageHandler, userData: *mut c_void),
    pub setThreadCount: extern "C" fn(threads: c_int, core: *mut VSCore) -> c_int,
    pub getPluginPath: extern "C" fn(plugin: *const VSPlugin) -> *const c_char,
    pub propGetIntArray: extern "C" fn(map: *const VSMap, key: *const c_char, error: *mut c_int)
                                       -> *const c_longlong,
    pub propGetFloatArray: extern "C" fn(map: *const VSMap, key: *const c_char, error: *mut c_int)
                                         -> *const f64,
    pub propSetIntArray: extern "C" fn(map: *mut VSMap,
                                       key: *const c_char,
                                       i: *const i64,
                                       size: c_int)
                                       -> c_int,
    pub propSetFloatArray: extern "C" fn(map: *mut VSMap,
                                         key: *const c_char,
                                         d: *const f64,
                                         size: c_int)
                                         -> c_int,
    pub logMessage: extern "C" fn(msgType: c_int, msg: *const c_char),
}
#[test]
fn bindgen_test_layout_VSAPI() {
    assert_eq!(::std::mem::size_of::<VSAPI>(), 592usize);
    assert_eq!(::std::mem::align_of::<VSAPI>(), 8usize);
}
impl Clone for VSAPI {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct VSFormat {
    pub name: [c_char; 32usize],
    pub id: c_int,
    pub colorFamily: c_int,
    pub sampleType: c_int,
    pub bitsPerSample: c_int,
    pub bytesPerSample: c_int,
    pub subSamplingW: c_int,
    pub subSamplingH: c_int,
    pub numPlanes: c_int,
}
#[test]
fn bindgen_test_layout_VSFormat() {
    assert_eq!(::std::mem::size_of::<VSFormat>(), 64usize);
    assert_eq!(::std::mem::align_of::<VSFormat>(), 4usize);
}
impl Clone for VSFormat {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct VSCoreInfo {
    pub versionString: *const c_char,
    pub core: c_int,
    pub api: c_int,
    pub numThreads: c_int,
    pub maxFramebufferSize: i64,
    pub usedFramebufferSize: i64,
}
#[test]
fn bindgen_test_layout_VSCoreInfo() {
    assert_eq!(::std::mem::size_of::<VSCoreInfo>(), 40usize);
    assert_eq!(::std::mem::align_of::<VSCoreInfo>(), 8usize);
}
impl Clone for VSCoreInfo {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct VSVideoInfo {
    pub format: *const VSFormat,
    pub fpsNum: i64,
    pub fpsDen: i64,
    pub width: c_int,
    pub height: c_int,
    pub numFrames: c_int,
    pub flags: c_int,
}
#[test]
fn bindgen_test_layout_VSVideoInfo() {
    assert_eq!(::std::mem::size_of::<VSVideoInfo>(), 40usize);
    assert_eq!(::std::mem::align_of::<VSVideoInfo>(), 8usize);
}
impl Clone for VSVideoInfo {
    fn clone(&self) -> Self {
        *self
    }
}

pub type VSGetVapourSynthAPI = extern "C" fn(version: c_int) -> *const VSAPI;
pub type VSPublicFunction = extern "C" fn(in_: *const VSMap,
                                          out: *mut VSMap,
                                          userData: *mut c_void,
                                          core: *mut VSCore,
                                          vsapi: *const VSAPI);
pub type VSRegisterFunction = extern "C" fn(name: *const c_char,
                                            args: *const c_char,
                                            argsFunc: VSPublicFunction,
                                            functionData: *mut c_void,
                                            plugin: *mut VSPlugin);
pub type VSConfigPlugin = extern "C" fn(identifier: *const c_char,
                                        defaultNamespace: *const c_char,
                                        name: *const c_char,
                                        apiVersion: c_int,
                                        readonly: c_int,
                                        plugin: *mut VSPlugin);
pub type VSInitPlugin = extern "C" fn(configFunc: VSConfigPlugin,
                                      registerFunc: VSRegisterFunction,
                                      plugin: *mut VSPlugin);
pub type VSFreeFuncData = extern "C" fn(userData: *mut c_void);
pub type VSFilterInit = extern "C" fn(in_: *mut VSMap,
                                      out: *mut VSMap,
                                      instanceData: *mut *mut c_void,
                                      node: *mut VSNode,
                                      core: *mut VSCore,
                                      vsapi: *const VSAPI);
pub type VSFilterGetFrame = extern "C" fn(n: c_int,
                                          activationReason: c_int,
                                          instanceData: *mut *mut c_void,
                                          frameData: *mut *mut c_void,
                                          frameCtx: *mut VSFrameContext,
                                          core: *mut VSCore,
                                          vsapi: *const VSAPI)
                                          -> *const VSFrameRef;
pub type VSFilterFree = extern "C" fn(instanceData: *mut c_void,
                                      core: *mut VSCore,
                                      vsapi: *const VSAPI);
pub type VSFrameDoneCallback = extern "C" fn(userData: *mut c_void,
                                             f: *const VSFrameRef,
                                             n: c_int,
                                             arg1: *mut VSNodeRef,
                                             errorMsg: *const c_char);
pub type VSMessageHandler = extern "C" fn(msgType: c_int,
                                          msg: *const c_char,
                                          userData: *mut c_void);
extern "C" {
    pub fn getVapourSynthAPI(version: c_int) -> *const VSAPI;
}