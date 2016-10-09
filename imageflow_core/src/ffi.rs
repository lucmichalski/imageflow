//! # Do not use
//! Do not use functions from this module outside of imageflow_core
//!
//! **Use imageflow_core::abi functions instead when creating bindings**
//!
//! These aren't to be exposed, but rather to connect to C internals
extern crate imageflow_serde as s;
extern crate libc;
use std::ascii::AsciiExt;
use std::ptr;

use std::str::FromStr;
use libc::{c_void,c_float,int32_t,int64_t,size_t,uint32_t};

use flow;

pub enum JobIO {}



#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum NodeType {
    Null = 0,
    primitive_Flip_Vertical_Mutate = 1,
    primitive_Flip_Horizontal_Mutate = 2,
    primitive_Crop_Mutate_Alias = 3,
    primitive_CopyRectToCanvas = 4, // Overwrite only, no compositing
    Create_Canvas = 5,
    primitive_RenderToCanvas1D = 6,
    primitive_Scale2D_RenderToCanvas1D = 7,
    primitive_bitmap_bgra_pointer,
    primitive_decoder,
    primitive_encoder,

    Fill_Rect_Mutate,
    non_primitive_nodes_begin = 256,

    Expand_Canvas,
    Transpose,
    Flip_Vertical,
    Flip_Horizontal,
    Render1D,
    Crop,
    Apply_Orientation,
    non_optimizable_nodes_begin = 512,

    Clone,
    decoder,
    encoder,

    Rotate_90,
    Rotate_180,
    Rotate_270,
    Scale, //(preserve colorspace), interpolation filter
    Noop,

    // Not implemented below here:
    Rotate_Flip_Per_Orientation,
    Crop_Percentage,
    Crop_Percentage_Infinite_Canvas, // canvas_color
    Crop_Rectangle,
    Constrain, //(mode=pad|max|crop|stretch) (width, height) (scale=down|up|both|canvas) (anchor=9 points)
    Matte,
    EnlargeCanvas,
    Sharpen,
    Blur,
    Convolve_Custom,
    AdjustContrast,
    AdjustSaturation,
    AdjustBrightness,
    CropWhitespace, // tolerances and padding
    Opacity,
    Sepia,
    Grayscale, // true|y|ry|ntsc|bt709|flat
    DrawImage,
    RemoveNoise,
    ColorMatrixsRGB,
    _FORCE_ENUM_SIZE_INT32 = 2147483647,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct CodecInstance {
    graph_placeholder_id: int32_t,
    codec_id: int64_t,
    codec_state: *mut c_void,
    io: *mut FlowIO,
    next: *mut CodecInstance,
    direction: FlowDirection,
}

#[repr(C)]
pub struct Job {
    pub debug_job_id: int32_t,
    pub next_stable_node_id: int32_t,
    pub next_graph_version: int32_t,
    pub max_calc_flatten_execute_passes: int32_t,
    //FIXME: find a safer way to store them
    pub codecs_head:           *mut CodecInstance,
    pub codecs_tail:           *mut CodecInstance,
    pub record_graph_versions: bool,
    pub record_frame_images:   bool,
    pub render_graph_versions: bool,
    pub render_animated_graph: bool,
    pub render_last_graph:     bool,
}

#[repr(C)]
pub enum IoMode {
    None = 0,
    read_sequential = 1,
    write_sequential = 2,
    read_seekable = 5, // 1 | 4,
    write_seekable = 6, // 2 | 4,
    read_write_seekable = 15, // 1 | 2 | 4 | 8
}
#[repr(C)]
#[derive(Copy,Clone)]
pub enum IoDirection {
    Out = 8,
    In = 4,
}


#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum EdgeKind {
    //None = 0, In the managed version, we don't need None edges
    Input = 1,
    Canvas = 2,
    Info = 3,
}

/*
#[repr(C)]
#[derive(Copy,Clone, Debug)]
pub enum PixelFormat {
    bgr24 = 3,
    bgra32 = 4,
    gray8 = 1,
}
*/

#[repr(C)]
#[derive(Copy,Clone, Debug)]
pub enum Floatspace {
    srgb = 0,
    linear = 1,
    gamma = 2,
}

/*
#[repr(C)]
#[derive(Copy,Clone, Debug)]
pub enum BitmapCompositingMode {
    replace_with_self = 0,
    blend_with_self = 1,
    blend_with_matte = 2,
}
*/

//
//#[repr(C)]
//#[derive(Copy,Clone,Debug, PartialEq)]
//pub enum Filter {
//    RobidouxFast = 1,
//    Robidoux = 2,
//    RobidouxSharp = 3,
//    Ginseng = 4,
//    GinsengSharp = 5,
//    Lanczos = 6,
//    LanczosSharp = 7,
//    Lanczos2 = 8,
//    Lanczos2Sharp = 9,
//    CubicFast = 10,
//    Cubic = 11,
//    CubicSharp = 12,
//    CatmullRom = 13,
//    Mitchell = 14,
//
//    CubicBSpline = 15,
//    Hermite = 16,
//    Jinc = 17,
//    RawLanczos3 = 18,
//    RawLanczos3Sharp = 19,
//    RawLanczos2 = 20,
//    RawLanczos2Sharp = 21,
//    Triangle = 22,
//    Linear = 23,
//    Box = 24,
//    CatmullRomFast = 25,
//    CatmullRomFastSharp = 26,
//
//    Fastest = 27,
//
//    MitchellFast = 28,
//    NCubic = 29,
//    NCubicSharp = 30,
//}

pub use self::s::Filter as Filter;

#[repr(C)]
#[derive(Copy,Clone,Debug, PartialEq)]
pub enum FlowStatusCode {
    NoError                    = 0,
    OutOfMemory                = 10,
    IOError                    = 20,
    InvalidInternalState       = 30,
    NotImplemented             = 40,
    InvalidArgument            = 50,
    NullArgument               = 51,
    InvalidDimensions          = 52,
    UnsupportedPixelFormat     = 53,
    ItemDoesNotExist           = 54,

    ImageDecodingFailed        = 60,
    ImageEncodingFailed        = 61,
    GraphInvalid               = 70,
    GraphIsCyclic              = 71,
    InvalidInputsToNode        = 72,
    MaximumGraphPassesExceeded = 73,
    OtherError                 = 1024,
    //FIXME: FirstUserDefinedError is 1025 in C but it conflicts with __LastLibraryError
    //___LastLibraryError,
    FirstUserDefinedError      = 1025,
    LastUserDefinedError       = 2147483647,
}


pub const TESTED_FILTER_OPTIONS: &'static [&'static str] = &["",
                                                             "robidoux",
                                                             "robidouxsharp",
                                                             "ginseng",
                                                             "lanczos",
                                                             "lanczos2",
                                                             "catmullrom",
                                                             "catrom",
                                                             "mitchell",
                                                             "cubicbspline",
                                                             "bspline",
                                                             "hermite",
                                                             "triangle",
                                                             "ncubic",
                                                             "ncubicsharp"];

pub const FILTER_OPTIONS: &'static [&'static str] = &["robidouxfast",
                                                      "robidoux",
                                                      "robidouxsharp",
                                                      "ginseng",
                                                      "ginsengsharp",
                                                      "lanczos",
                                                      "lanczossharp",
                                                      "lanczos2",
                                                      "lanczos2sharp",
                                                      "cubicfast",
                                                      "cubic",
                                                      "cubicsharp",
                                                      "catmullrom",
                                                      "catrom",
                                                      "mitchell",
                                                      "cubicbspline",
                                                      "bspline",
                                                      "hermite",
                                                      "jinc",
                                                      "rawlanczos3",
                                                      "rawlanczos3sharp",
                                                      "rawlanczos2",
                                                      "rawlanczos2sharp",
                                                      "triangle",
                                                      "linear",
                                                      "box",
                                                      "catmullromfast",
                                                      "catmullromfastsharp",
                                                      "fastest",
                                                      "mitchellfast",
                                                      "ncubic",
                                                      "ncubicsharp"];



impl Default for DecoderInfo {
    fn default() -> DecoderInfo {
        DecoderInfo {
            codec_id: -1,
            preferred_mime_type: ptr::null(),
            preferred_extension: ptr::null(),
            frame_count: 0,
            current_frame_index: 0,
            frame0_width: 0,
            frame0_height: 0,
            frame0_post_decode_format: PixelFormat::BGRA32,
        }
    }
}

#[repr(C)]
pub struct DecoderInfo {
    pub codec_id: i64,
    pub preferred_mime_type: *const i8,
    pub preferred_extension: *const i8,
    pub frame_count: usize,
    pub current_frame_index: i64,
    pub frame0_width: i32,
    pub frame0_height: i32,
    pub frame0_post_decode_format: PixelFormat,
}



#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct BitmapBgra {
    /// bitmap width in pixels
    pub w: uint32_t,
    /// bitmap height in pixels
    pub h: uint32_t,
    /// byte length of each row (may include any amount of padding)
    pub stride: uint32_t,
    //FIXME: replace with a vec or slice
    ///pointer to pixel 0,0; should be of length > h * stride
    pub pixels: *mut u8,
    /// If true, we don't dispose of *pixels when we dispose the struct
    pub borrowed_pixels: bool,
    /// If false, we can even ignore the alpha channel on 4bpp
    pub alpha_meaningful: bool,
    /// If false, we can edit pixels without affecting the stride
    pub pixels_readonly: bool,
    ///If false, we can change the stride of the image
    pub stride_readonly: bool,
    /// If true, we can reuse the allocated memory for other purposes
    pub can_reuse_space: bool,
    pub fmt: PixelFormat,
    ///When using compositing mode blend_with_matte, this color will be used. We should probably define this as
    ///always being sRGBA, 4 bytes.
    pub matte_color: [u8;4],

    pub compositing_mode: BitmapCompositingMode,
}
/*imageflow_core::ffi::FlowBitmapBgra{
        alpha_meaningful: false,
        can_reuse_space: false,
        compositing_mode: ffi::BitmapCompositingMode::blend_with_self,
        matte_color: [0,0,0,0],
        pixels_readonly: false,
        stride_readonly: false,
        pixels: ptr::null_mut(),
        stride: 0,
        w: 0,
        h: 0,
        borrowed_pixels: false,
        fmt: ffi::PixelFormat::bgra32
    };*/



#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum PixelFormat {
    Gray8  = 1,
    BGR24  = 3,
    BGRA32 = 4,
}

impl From<s::PixelFormat> for PixelFormat{
    fn from(f: s::PixelFormat) -> PixelFormat{
        match f {
            s::PixelFormat::Bgr24 => PixelFormat::BGR24,
            s::PixelFormat::Bgra32 => PixelFormat::BGRA32,
            s::PixelFormat::Gray8 => PixelFormat::Gray8
        }
    }
}
impl <'a> From<&'a s::PixelFormat> for PixelFormat{
    fn from(f: &'a s::PixelFormat) -> PixelFormat{
        match *f {
            s::PixelFormat::Bgr24 => PixelFormat::BGR24,
            s::PixelFormat::Bgra32 => PixelFormat::BGRA32,
            s::PixelFormat::Gray8 => PixelFormat::Gray8
        }
    }
}
impl From<PixelFormat> for s::PixelFormat{
    fn from(f: PixelFormat) -> s::PixelFormat{
        match f {
            PixelFormat::BGR24 => s::PixelFormat::Bgr24,
            PixelFormat::BGRA32 => s::PixelFormat::Bgra32,
            PixelFormat::Gray8 => s::PixelFormat::Gray8
        }
    }
}



#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum EdgeType {
    Null   = 0,
    Input  = 1,
    Canvas = 2,
    info   = 3,
    FORCE_ENUM_SIZE_INT32 = 2147483647,
}

#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum BitmapCompositingMode {
    ReplaceSelf    = 0,
    BlendWithSelf  = 1,
    BlendWithMatte = 2,
}



/// floating-point bitmap, typically linear RGBA, premultiplied
#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct BitmapFloat {
    /// buffer width in pixels
    w: uint32_t,
    /// buffer height in pixels
    h: uint32_t,
    /// The number of floats per pixel
    channels: uint32_t,
    /// The pixel data
    pixels: *mut c_float,
    /// If true, don't dispose the buffer with the struct
    pixels_borrowed: bool,
    /// The number of floats in the buffer
    float_count: uint32_t,
    /// The number of floats betwen (0,0) and (0,1)
    float_stride: uint32_t,

    /// If true, alpha has been premultiplied
    alpha_premultiplied: bool,
    /// If true, the alpha channel holds meaningful data
    alpha_meaningful: bool,
}

/** flow_context: Heap Manager **/

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct Heap {
    placeholder: u8,
    /*FIXME: fill in the rest
    flow_heap_calloc_function _calloc;
    flow_heap_malloc_function _malloc;
    flow_heap_realloc_function _realloc;
    flow_heap_free_function _free;
    flow_heap_terminate_function _context_terminate;
    void * _private_state;
*/
}

//struct flow_objtracking_info;
//void flow_context_objtracking_initialize(struct flow_objtracking_info * heap_tracking);
//void flow_context_objtracking_terminate(flow_c * c);

/** flow_context: struct flow_error_info **/

/*
struct flow_error_callstack_line {
    const char * file;
    int line;
    const char * function_name;
};
*/

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct ErrorInfo {
    placeholder: u8,
    /*FIXME: replace
    flow_status_code reason;
    struct flow_error_callstack_line callstack[14];
    int callstack_count;
    int callstack_capacity;
    bool locked;
    char message[FLOW_ERROR_MESSAGE_SIZE + 1];
*/
}

/*
#ifdef EXPOSE_SIGMOID
// flow_context: Colorspace
struct flow_SigmoidInfo {
    float constant;
    float x_coeff;
    float x_offset;
    float y_offset;
    float y_coeff;
};
#endif
*/

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct ColorspaceInfo {
    placeholder: u8,
    /*FIXME: replace
    float byte_to_float[256]; // Converts 0..255 -> 0..1, but knowing that 0.255 has sRGB gamma.
    flow_working_floatspace floatspace;
    bool apply_srgb;
    bool apply_gamma;
    float gamma;
    float gamma_inverse;
#ifdef EXPOSE_SIGMOID
    struct flow_SigmoidInfo sigmoid;
    bool apply_sigmoid;
#endif
*/
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct HeapObjectRecord {
    placeholder: u8,
    /*FIXME: fill in the rest
    void * ptr;
    size_t bytes;
    void * owner;
    flow_destructor_function destructor;
    bool destructor_called;
    const char * allocated_by;
    int allocated_by_line;
    bool is_owner;
*/
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct ObjTrackingInfo {
    pub allocs: HeapObjectRecord,
    pub next_free_slot: size_t,
    pub total_slots: size_t,
    pub bytes_allocated_net: size_t,
    pub bytes_allocated_gross: size_t,
    pub allocations_net: size_t,
    pub allocations_gross: size_t,
    pub bytes_free: size_t,
    pub allocations_net_peak: size_t,
    pub bytes_allocations_net_peak: size_t,
}


#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct CodecDefinition {
    placeholder: u8,
    //FIXME: replace
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct ContextCodecSet {
    //FIXME: replace with a Vec?
    codecs: *mut CodecDefinition,
    codecs_count: size_t,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct ContextNodeSet {
    //FIXME: replace with a Vec?
    codecs: *mut c_void,
    codecs_count: size_t,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct ProfilingLog {
    placeholder: u8,
    //FIXME: replace
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct Context {
    pub error: ErrorInfo,
    pub underlying_heap: Heap,
    pub log: ProfilingLog,
    pub colorspace: ColorspaceInfo,
    pub object_tracking: ObjTrackingInfo,
    pub codec_set: *mut ContextCodecSet,
    pub node_set:  *mut ContextNodeSet,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct NodeInfoIndex {
    pub index: int32_t,
}

#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum CodecType {
    Null       = 0,
    DecodePng  = 1,
    EncodePng  = 2,
    DecodeJpeg = 3,
    EncodeJpeg = 4,
    DecodeGif  = 5,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
struct NodeInfoEncoderPlaceholder {
    index: NodeInfoIndex,
    codec_type: CodecType,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
struct NodeInfoCreateCanvas {
    format: PixelFormat,
    width: size_t,
    height: size_t,
    bgcolor: uint32_t,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
struct NodeInfoCrop {
    x1: uint32_t,
    x2: uint32_t,
    y1: uint32_t,
    y2: uint32_t,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
struct NodeInfoCopyRectToCanvas {
    x: uint32_t,
    y: uint32_t,
    from_x: uint32_t,
    from_y: uint32_t,
    width: uint32_t,
    height: uint32_t,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
struct NodeInfoExpandCanvas {
    left: uint32_t,
    top: uint32_t,
    right: uint32_t,
    bottom: uint32_t,
    canvas_color_srgb: uint32_t,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
struct NodeInfoFillRect {
    x1: uint32_t,
    y1: uint32_t,
    x2: uint32_t,
    y2: uint32_t,
    color_srgb: uint32_t,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
struct NodeInfoSize {
    width: int32_t,
    height: int32_t,
}

#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
enum InterpolationFilter {
    RobidouxFast = 1,
    Robidoux = 2,
    RobidouxSharp = 3,
    Ginseng = 4,
    GinsengSharp = 5,
    Lanczos = 6,
    LanczosSharp = 7,
    Lanczos2 = 8,
    Lanczos2Sharp = 9,
    CubicFast = 10,
    Cubic = 11,
    CubicSharp = 12,
    CatmullRom = 13,
    Mitchell = 14,

    CubicBSpline = 15,
    Hermite = 16,
    Jinc = 17,
    RawLanczos3 = 18,
    RawLanczos3Sharp = 19,
    RawLanczos2 = 20,
    RawLanczos2Sharp = 21,
    Triangle = 22,
    Linear = 23,
    Box = 24,
    CatmullRomFast = 25,
    CatmullRomFastSharp = 26,

    Fastest = 27,

    MitchellFast = 28,

    NCubic = 29,

    NCubicSharp = 30
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
struct NodeInfoScale {
    width: int32_t,
    height: int32_t,
    downscale_filter: InterpolationFilter,
    upscale_filter:   InterpolationFilter,
    flags: size_t,
    sharpen: c_float,
}

#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
enum ScaleFlags {
    None = 0,
    UseScale2d = 1,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct NodeInfoBitmapBgraPointer {
    ptr: *mut *mut BitmapBgra,
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct DecoderDownscaleHints {
    downscale_if_wider_than: int64_t,
    or_if_taller_than: int64_t,
    downscaled_min_width: int64_t,
    downscaled_min_height: int64_t,
    scale_luma_spatially: bool,
    gamma_correct_for_srgb_during_spatial_luma_scaling: bool
}

#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct EncoderHints {
    pub jpeg_encode_quality: int32_t,
    pub disable_png_alpha: bool,
}

// If you want to know what kind of I/O structure is inside user_data, compare the read_func/write_func function
// pointers. No need for another human-assigned set of custom structure identifiers.
#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
struct FlowIO {
    placeholder: u8,
    /*
        flow_c * context;
        flow_io_mode mode; // Call nothing, dereference nothing, if this is 0
        flow_io_read_function read_func; // Optional for write modes
        flow_io_write_function write_func; // Optional for read modes
        flow_io_position_function position_func; // Optional for sequential modes
        flow_io_seek_function seek_function; // Optional for sequential modes
        flow_destructor_function dispose_func; // Optional.
        void * user_data;
        int64_t optional_file_length; // Whoever sets up this structure can populate this value - or set it to -1 - as they
        // wish. useful for resource estimation.
    */
}

#[repr(C)]
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum FlowDirection {
    Output = 8,
    Input = 4,
}



#[repr(C)]
#[derive(Clone,Debug,PartialEq)]
pub struct NodeInfoCodec {
    placeholder_id: int32_t,
    codec: *mut CodecInstance,
    // For encoders
    desired_encoder_id: int64_t,
    // For decoders
    downscale_hints: DecoderDownscaleHints,
    encoder_hints:   EncoderHints,
}


#[repr(C)]
#[derive(Clone,Debug,Copy)]
pub struct Scale2dRenderToCanvas1d {
    // There will need to be consistency checks against the createcanvas node

    // struct flow_interpolation_details * interpolationDetails;
    pub scale_to_width: i32,
    pub scale_to_height: i32,
    pub sharpen_percent_goal: f32,
    pub interpolation_filter: Filter,
    pub scale_in_colorspace: Floatspace,
}

extern "C" {
    pub fn flow_context_create() -> *mut Context;
    pub fn flow_context_begin_terminate(context: *mut Context) -> bool;
    pub fn flow_context_destroy(context: *mut Context);
    pub fn flow_context_has_error(context: *mut Context) -> bool;
    pub fn flow_context_clear_error(context: *mut Context);
    pub fn flow_context_error_and_stacktrace(context: *mut Context,
                                             buffer: *mut u8,
                                             buffer_length: libc::size_t,
                                             full_file_path: bool)
                                             -> i64;
    pub fn flow_context_print_and_exit_if_err(context: *mut Context) -> bool;

    pub fn flow_context_error_reason(context: *mut Context) -> i32;

    pub fn flow_context_set_error_get_message_buffer(context: *mut Context, code: i32/*FlowStatusCode*/,
                                                     file: *const libc::c_char, line: i32, function_name: *const libc::c_char) -> *const libc::c_char;

    pub fn flow_context_raise_error(context: *mut Context,
                                    error_code: i32,
                                    message: *const libc::c_char,
                                    file: *const libc::c_char,
                                    line: i32,
                                    function_name: *const libc::c_char)
                                    -> bool;


    pub fn flow_context_add_to_callstack(context: *mut Context,
                                         file: *const libc::c_char,
                                         line: i32,
                                         function_name: *const libc::c_char)
                                         -> bool;



    pub fn flow_context_calloc(context: *mut Context,
                               instance_count: usize,
                               instance_size: usize,
                               destructor: *const libc::c_void,
                               owner: *const libc::c_void,
                               file: *const libc::c_char,
                               line: i32)
                               -> *mut libc::c_void;

    pub fn flow_destroy(context: *mut Context,
                        pointer: *const libc::c_void,
                        file: *const libc::uint8_t,
                        line: i32)
                        -> bool;

    pub fn flow_job_destroy(context: *mut Context, job: *mut Job) -> bool;




    pub fn flow_job_create(context: *mut Context) -> *mut Job;


    pub fn flow_job_configure_recording(context: *mut Context,
                                        job: *mut Job,
                                        record_graph_versions: bool,
                                        record_frame_images: bool,
                                        render_last_graph: bool,
                                        render_graph_versions: bool,
                                        render_animated_graph: bool)
                                        -> bool;





    pub fn flow_io_create_for_file(context: *mut Context,
                                   mode: IoMode,
                                   filename: *const libc::c_char,
                                   owner: *const libc::c_void)
                                   -> *mut JobIO;

    pub fn flow_io_create_from_memory(context: *mut Context,
                                      mode: IoMode,
                                      memory: *const u8,
                                      length: libc::size_t,
                                      owner: *const libc::c_void,
                                      destructor_function: *const libc::c_void)
                                      -> *mut JobIO;

    pub fn flow_io_create_for_output_buffer(context: *mut Context,
                                            owner: *const libc::c_void)
                                            -> *mut JobIO;


    // Returns false if the flow_io struct is disposed or not an output buffer type (or for any other error)
    //
    pub fn flow_io_get_output_buffer(context: *mut Context,
                                     io: *mut JobIO,
                                     result_buffer: *mut *const u8,
                                     result_buffer_length: *mut libc::size_t)
                                     -> bool;

    pub fn flow_job_get_io(context: *mut Context,
                           job: *mut Job,
                           placeholder_id: i32)
                           -> *mut JobIO;



    pub fn flow_job_add_io(context: *mut Context,
                           job: *mut Job,
                           io: *mut JobIO,
                           placeholder_id: i32,
                           direction: IoDirection)
                           -> bool;

    pub fn flow_job_get_decoder_info(c: *mut Context,
                                     job: *mut Job,
                                     by_placeholder_id: i32,
                                     info: *mut DecoderInfo)
                                     -> bool;




    pub fn flow_job_decoder_set_downscale_hints_by_placeholder_id(c: *mut Context,
                                                                  job: *mut Job, placeholder_id: i32,
                                                                  if_wider_than: i64, or_taller_than: i64,
                                                                  downscaled_min_width: i64, downscaled_min_height: i64, scale_luma_spatially: bool,
                                                                  gamma_correct_for_srgb_during_spatial_luma_scaling: bool) -> bool;


    pub fn flow_context_set_floatspace(c: *mut Context,
                                       space: Floatspace,
                                       a: f32,
                                       b: f32,
                                       c: f32);

    pub fn flow_bitmap_bgra_test_compare_to_record(c: *mut Context,
                                                   bitmap: *mut BitmapBgra,
                                                   storage_name: *const libc::c_char,
                                                   store_if_missing: bool,
                                                   off_by_one_byte_differences_permitted: usize,
                                                   caller_filename: *const libc::c_char,
                                                   caller_linenumber: i32,
                                                   storage_relative_to: *const libc::c_char)
                                                   -> bool;


    pub fn flow_bitmap_bgra_flip_vertical(c: *mut Context,
                                          bitmap: *mut BitmapBgra);
    pub fn flow_bitmap_bgra_flip_horizontal(c: *mut Context,
                                            bitmap: *mut BitmapBgra);

    pub fn flow_bitmap_bgra_create(c: *mut Context,
                                   sx: i32, sy: i32, zeroed: bool, format: PixelFormat) -> *mut BitmapBgra;


    pub fn flow_node_execute_scale2d_render1d(c: *mut Context,
                                              job: *mut Job, input: *mut BitmapBgra, canvas: *mut BitmapBgra, info: *const Scale2dRenderToCanvas1d) -> bool;

    pub fn flow_bitmap_bgra_fill_rect(c: *mut Context, input: *mut BitmapBgra, x1: u32, y1: u32, x2: u32, y2: u32, color_srgb_argb: u32) -> bool;
}


// https://github.com/rust-lang/rust/issues/17417


#[test]
fn flow_context_create_destroy_works() {
    unsafe {
        let c = flow_context_create();
        assert!(!c.is_null());

        flow_context_destroy(c);
    }
}

#[test]
fn flow_job_creation_works() {
    unsafe {
        let c = flow_context_create();
        assert!(!c.is_null());

        let j = flow_job_create(c);
        assert!(!j.is_null());

        flow_context_destroy(c);
    }
}

