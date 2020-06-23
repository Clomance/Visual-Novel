use crate::{
    Colour,
    graphics::GraphicsSettings,
};

use glium::glutin::{
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

#[derive(Clone,Debug)]
#[allow(dead_code)]
pub struct WindowSettings{
    //--General attributes--\\

    /// Whether the window should be filled with given colour upon creation.
    /// 
    /// The default is None.
    pub initial_colour:Option<Colour>,



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

    /// If true, only sRGB-capable formats will be considered.
    /// If false, don't care.
    /// 
    /// The default is true.
    pub srgb:bool,

    /// If true, only hardware-accelerated formats will be considered.
    /// If false, only software renderers.
    /// None means "don't care".
    /// 
    /// Default is Some(true).
    pub hardware_accelerated:Option<bool>,



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
        Self{
            //--General attributes--\\
            initial_colour:Option::None,



            //--Window attributes--\\
            inner_size:Option::None,
            min_inner_size:Option::None,
            max_inner_size:Option::None,
            resizable:true,
            fullscreen:Option::None,
            title:"Window".to_string(),
            maximized:false,
            visible:true,
            transparent:true,
            decorations:true,
            always_on_top:false,
            window_icon:Option::None,



            //--OpenGL attributes--\\
            debug:false,
            vsync:false,



            //--Pixel format requirements--\\
            srgb:true,
            hardware_accelerated:Option::None,



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
        context_builder.pf_reqs.srgb=self.srgb;

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