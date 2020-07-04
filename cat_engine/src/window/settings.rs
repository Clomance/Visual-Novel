use crate::{
    Colour,
    graphics::GraphicsSettings,
};

use glium::glutin::{
    ReleaseBehavior,
    NotCurrent,
    ContextBuilder,
    dpi::Size,
    window::{
        WindowBuilder,
        WindowAttributes,
        Fullscreen,
        Icon
    },
};

use std::{
    ops::Range,
    path::PathBuf
};

#[derive(Clone,Debug)]
#[allow(dead_code)]
pub struct WindowSettings{
    //--General attributes--\\

    /// Whether the window should be filled with given colour upon creation.
    /// 
    /// The default is None.
    pub initial_colour:Option<Colour>,

    /// The path for the mouse cursor icon.
    /// 
    /// The default is `./mouse_cursor_icon.png`.
    /// 
    /// feature = "mouse_cursor_icon"
    #[cfg(feature="mouse_cursor_icon")]
    pub mouse_cursor_icon_path:PathBuf,

    /// The range of the texture vertex buffer to save
    /// the mouse cursor vertexes.
    /// 
    /// The default is 4..8.
    /// 
    /// feature = "mouse_cursor_icon"
    #[cfg(feature="mouse_cursor_icon")]
    pub mouse_cursor_icon_range:Range<usize>,

    //--Window attributes--\\

    /// The dimensions of the window.
    /// If this is None, some platform-specific dimensions will be used.
    /// 
    /// The default is None.
    pub inner_size:Option<Size>,

    /// The minimum dimensions a window can be.
    /// If this is None, the window will have no minimum dimensions (aside from reserved).
    /// 
    /// The default is None.
    pub min_inner_size:Option<Size>,

    /// The maximum dimensions a window can be.
    /// If this is None, the maximum will have no maximum or will be set to the primary monitor's dimensions by the platform.
    /// 
    /// The default is None.
    pub max_inner_size:Option<Size>,

    /// Whether the window is resizable or not.
    /// 
    /// The default is true.
    pub resizable:bool,

    /// Whether the window should be set as fullscreen upon creation.
    /// 
    /// The default is None.
    pub fullscreen:Option<Fullscreen>,

    /// The title of the window in the title bar.
    /// 
    /// The default is "Window".
    pub title:String,

    /// Whether the window should be maximized upon creation.
    /// 
    /// The default is false.
    pub maximized:bool,

    /// Whether the window should be immediately visible upon creation.
    /// 
    /// The default is true.
    pub visible:bool,

    /// Whether the the window should be transparent.
    /// If this is true, writing colors with alpha values different than 1.0 will produce a transparent window.
    /// 
    /// The default is false.
    pub transparent:bool,

    /// Whether the window should have borders and bars.
    /// 
    /// The default is true.
    pub decorations:bool,

    /// Whether the window should always be on top of other windows.
    /// 
    /// The default is false.
    pub always_on_top:bool,

    /// The window icon.
    /// 
    /// The default is None.
    pub window_icon:Option<Icon>,



    //--OpenGL attributes--\\

    /// Whether to enable the debug flag of the context.
    /// 
    /// Debug contexts are usually slower but give better error reporting.
    /// 
    /// The default is false.
    pub debug:bool,

    /// Whether to use vsync.
    /// If vsync is enabled, calling swap_buffers will block until the screen refreshes.
    /// This is typically used to prevent screen tearing.
    /// 
    /// The default is false.
    pub vsync:bool,



    //--Pixel format requirements--\\

    /// If true, only hardware-accelerated formats will be considered.
    /// If false, only software renderers.
    /// None means "don't care".
    /// 
    /// Default is Some(true).
    pub hardware_accelerated:Option<bool>,

    /// Minimum number of bits for the color buffer, excluding alpha.
    /// None means "don't care".
    /// 
    /// The default is Some(24).
    pub color_bits:Option<u8>,

    /// If true, the color buffer must be in a floating point format.
    /// Using floating points allows you to write values outside of the [0.0, 1.0] range.
    /// 
    /// Default is false.
    pub float_color_buffer:bool,

    /// Minimum number of bits for the alpha in the color buffer.
    /// None means "don't care".
    /// 
    /// The default is Some(8).
    pub alpha_bits:Option<u8>,

    /// Minimum number of bits for the depth buffer.
    /// None means "don't care".
    /// 
    /// The default value is Some(24).
    pub depth_bits:Option<u8>,

    /// Minimum number of stencil bits.
    /// None means "don't care".
    /// 
    /// The default value is Some(8).
    pub stencil_bits:Option<u8>,

    /// If true, only double-buffered formats will be considered.
    /// If false, only single-buffer formats.
    /// None means "don't care".
    /// 
    /// The default is Some(true).
    pub double_buffer:Option<bool>,

    /// Contains the minimum number of samples per pixel in the color, depth and stencil buffers.
    /// None means "don't care".
    /// A value of Some(0) indicates that multisampling must not be enabled.
    /// 
    /// Default is None.
    pub multisampling:Option<u16>,

    /// If true, only stereoscopic formats will be considered.
    /// If false, only non-stereoscopic formats.
    /// 
    /// The default is false.
    pub stereoscopy:bool,

    /// If true, only sRGB-capable formats will be considered.
    /// If false, don't care.
    /// 
    /// The default is true.
    pub srgb:bool,


    /// The behavior when changing the current context.
    /// 
    /// Default is Flush.
    pub release_behavior:ReleaseBehavior,



    //--Local graphics attributes--\\

    /// The default is 8.
    /// 
    /// feature = "texture_graphics"
    #[cfg(feature="texture_graphics")]
    pub texture_vertex_buffer_size:usize,

    /// The default is 100.
    /// 
    /// feature = "simple_graphics"
    #[cfg(feature="simple_graphics")]
    pub simple_vertex_buffer_size:usize,

    /// The default is 2000.
    /// 
    /// feature = "text_graphics"
    #[cfg(feature="text_graphics")]
    pub text_vertex_buffer_size:usize,
}

#[allow(dead_code)]
impl WindowSettings{
    /// Default settings.
    pub fn new()->WindowSettings{

        #[cfg(feature="mouse_cursor_icon")]
        let mut path=PathBuf::new();
        #[cfg(feature="mouse_cursor_icon")]
        path.push("./mouse_cursor_icon.png");

        Self{
            //--General attributes--\\
            initial_colour:None,

            #[cfg(feature="mouse_cursor_icon")]
            mouse_cursor_icon_path:path,

            #[cfg(feature="mouse_cursor_icon")]
            mouse_cursor_icon_range:4..8,

            //--Window attributes--\\
            inner_size:None,
            min_inner_size:None,
            max_inner_size:None,
            resizable:true,
            fullscreen:None,
            title:"Window".to_string(),
            maximized:false,
            visible:true,
            transparent:true,
            decorations:true,
            always_on_top:false,
            window_icon:None,



            //--OpenGL attributes--\\
            debug:false,
            vsync:false,



            //--Pixel format requirements--\\
            hardware_accelerated:Option::None,
            color_bits:Some(24),
            float_color_buffer:false,
            alpha_bits:Some(8),
            depth_bits:Some(24),
            stencil_bits:Some(8),
            double_buffer:Some(true),
            multisampling:None,
            stereoscopy:false,
            srgb:true,
            release_behavior:ReleaseBehavior::Flush,


            //--Local graphics attributes--\\
            #[cfg(feature="texture_graphics")]
            texture_vertex_buffer_size:8usize,
            #[cfg(feature="simple_graphics")]
            simple_vertex_buffer_size:100usize,
            #[cfg(feature="text_graphics")]
            text_vertex_buffer_size:2000usize,
        }
    }

    pub (crate) fn devide<'a>(self)->(WindowBuilder,ContextBuilder<'a,NotCurrent>,GraphicsSettings){
        let window_attributes=WindowAttributes{
            inner_size:self.inner_size,
            min_inner_size:self.min_inner_size,
            max_inner_size:self.max_inner_size,
            resizable:self.resizable,
            fullscreen:self.fullscreen,
            title:self.title,
            maximized:self.maximized,
            visible:self.visible,
            transparent:self.transparent,
            decorations:self.decorations,
            always_on_top:self.always_on_top,
            window_icon:self.window_icon,
        };

        let mut window_builder=WindowBuilder::default();
        window_builder.window=window_attributes;

        let mut context_builder=ContextBuilder::new();
        context_builder.gl_attr.vsync=self.vsync;
        context_builder.gl_attr.debug=self.debug;

        context_builder.pf_reqs.hardware_accelerated=self.hardware_accelerated;
        context_builder.pf_reqs.color_bits=self.color_bits;
        context_builder.pf_reqs.float_color_buffer=self.float_color_buffer;
        context_builder.pf_reqs.alpha_bits=self.alpha_bits;
        context_builder.pf_reqs.depth_bits=self.depth_bits;
        context_builder.pf_reqs.stencil_bits=self.stencil_bits;
        context_builder.pf_reqs.double_buffer=self.double_buffer;
        context_builder.pf_reqs.multisampling=self.multisampling;
        context_builder.pf_reqs.stereoscopy=self.stereoscopy;
        context_builder.pf_reqs.srgb=self.srgb;
        context_builder.pf_reqs.release_behavior=self.release_behavior;
        
        let graphics_settings=GraphicsSettings{
            #[cfg(feature="texture_graphics")]
            texture_vertex_buffer_size:self.texture_vertex_buffer_size,
            #[cfg(feature="simple_graphics")]
            simple_vertex_buffer_size:self.simple_vertex_buffer_size,
            #[cfg(feature="text_graphics")]
            text_vertex_buffer_size:self.text_vertex_buffer_size,
        };

        (window_builder,context_builder,graphics_settings)
    }
}