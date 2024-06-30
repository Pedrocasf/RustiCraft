#![no_std]
#![no_main]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(panic_internals)]
#![feature(effects)]
#![feature(const_trait_impl)]
#![feature(rustc_private)]
extern crate lazy_static; 
extern crate alloc;
extern crate hashbrown;
extern crate spin;
extern crate nalgebra as na;
extern crate num_traits;
use num_traits::float::Float;
pub mod vertices;
pub mod block_list;
pub mod block_uvs;
pub mod rand;

use na::{
    Vector3
};
use block_list::*;
use vertices::*;
use core::{f32::{self}};
use psp::{Align16};
use psp::sys::{
    self, ScePspFVector3, DisplayPixelFormat, GuContextType, GuSyncMode, GuSyncBehavior,
    TextureFilter, TextureEffect, TextureColorComponent,
    FrontFaceDirection, ShadingModel, GuState, TexturePixelFormat, DepthFunc,
    ClearBuffer, MipmapLevel,CtrlMode,SceCtrlData,CtrlButtons,
};
use psp::vram_alloc::get_vram_allocator;
use psp::{BUF_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT};
psp::module!("RustiCraft", 1, 1);

// Both width and height, this is a square image.
const IMAGE_SIZE: usize = 256;

// The image data *must* be aligned to a 16 byte boundary.
const TEXTURE: Align16<[u8; IMAGE_SIZE * IMAGE_SIZE * 4]> = Align16(*include_bytes!("../textures/eldpack/terrain.raw.swizzled"));

static mut LIST: Align16<[u32; 0x100000]> = Align16([0; 0x100000]);
const ROT_SPEED:f32 = 1.0 * (3.14/180.0);
const WALK_SPEED:f32 = 0.5;
fn psp_main() {
    unsafe { psp_main_inner() }
}

unsafe fn psp_main_inner() {
    rand::seed_context();
    psp::enable_home_button();
    
    psp::sys::sceCtrlSetSamplingCycle(0);
    psp::sys::sceCtrlSetSamplingMode(CtrlMode::Analog);

    let mut allocator = get_vram_allocator().unwrap();
    let fbp0 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888);
    let fbp1 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888);
    let zbp = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444);
    let texture = allocator.alloc_texture_pixels(IMAGE_SIZE as u32, IMAGE_SIZE as u32, TexturePixelFormat::Psm8888);

    core::ptr::copy_nonoverlapping(TEXTURE.0.as_ptr(), texture.as_mut_ptr_direct_to_vram(), IMAGE_SIZE * IMAGE_SIZE * 4);

    sys::sceGumLoadIdentity();

    sys::sceGuInit();

    sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x100000] as *mut _);
    sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0.as_mut_ptr_from_zero() as _, BUF_WIDTH as i32);
    sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, fbp1.as_mut_ptr_from_zero() as _, BUF_WIDTH as i32);
    sys::sceGuDepthBuffer(zbp.as_mut_ptr_from_zero() as _, BUF_WIDTH as i32);
    sys::sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
    sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    sys::sceGuDepthRange(65535, 0);
    sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    sys::sceGuEnable(GuState::ScissorTest);
    sys::sceGuDepthFunc(DepthFunc::GreaterOrEqual);
    sys::sceGuEnable(GuState::DepthTest);
    sys::sceGuFrontFace(FrontFaceDirection::Clockwise);
    sys::sceGuShadeModel(ShadingModel::Smooth);
    sys::sceGuEnable(GuState::CullFace);
    sys::sceGuEnable(GuState::Texture2D);
    sys::sceGuEnable(GuState::ClipPlanes);
    sys::sceGuFinish();
    sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);

    psp::sys::sceDisplayWaitVblankStart();

    // run sample
    let pad_data = &mut SceCtrlData::default();
    let mut world = World::new(); 
    let mut position = Vector3::new(0.0,0.0,0.0);
    let mut horizontal_a:f32 = -0.0;
    let mut vertical_a:f32 = 0.0;
    sys::sceGuDisplay(true);
    loop {
        sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut [u32; 0x100000] as *mut _);

        // clear screen
        sys::sceGuClearColor(0xff554433);
        sys::sceGuClearDepth(0);
        sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::DEPTH_BUFFER_BIT);

        // setup matrices for cube

        sys::sceGumMatrixMode(sys::MatrixMode::Projection);
        sys::sceGumLoadIdentity();
        sys::sceGumPerspective(75.0, 16.0 / 9.0, 0.0, 1000.0);

        {    
            if pad_data.buttons.contains(CtrlButtons::CIRCLE){
                horizontal_a -= ROT_SPEED;
            }
            if pad_data.buttons.contains(CtrlButtons::SQUARE){
                horizontal_a += ROT_SPEED;
            }
            if pad_data.buttons.contains(CtrlButtons::TRIANGLE){
                vertical_a -= ROT_SPEED;
            }
            if pad_data.buttons.contains(CtrlButtons::CROSS){
                vertical_a += ROT_SPEED;
            }
            let direction = Vector3::<f32>::new(
                vertical_a.cos() * horizontal_a.sin(),
                vertical_a.sin(),
                vertical_a.cos() * horizontal_a.cos(),

            );
            let right= Vector3::<f32>::new(
                (horizontal_a-(3.14/2.0)).sin(),
                0.0,
                (horizontal_a-(3.14/2.0)).cos(),
            );
            if pad_data.buttons.contains(CtrlButtons::UP){
                position += direction * WALK_SPEED;
            }
            if pad_data.buttons.contains(CtrlButtons::DOWN){
                position -= direction * WALK_SPEED;
            }
            if pad_data.buttons.contains(CtrlButtons::RIGHT){
                position += right * WALK_SPEED;
            }
            if pad_data.buttons.contains(CtrlButtons::LEFT){
                position -= right * WALK_SPEED;
            }
            
            let up = Vector3::<f32>::new(
                (right.y * direction.z) - (right.z * direction.y),
                (right.z * direction.x) - (right.x * direction.z),
                (right.x * direction.y) - (right.y * direction.x)
            );
            let pmd = position+direction;
            let up = ScePspFVector3{x:up.x,y:up.y,z:up.z};
            let center = ScePspFVector3{x:pmd.x,y:pmd.y,z:pmd.z};
            let pos = ScePspFVector3{x:position.x,y:position.y,z:position.z};
            sys::sceGumMatrixMode(sys::MatrixMode::View);
            sys::sceGumLoadIdentity();
            sys::sceGumLookAt(&pos, &center, &up);
            sys::sceGumRotateXYZ(&ScePspFVector3{x:vertical_a,y:-horizontal_a,z:0.0});
            sys::sceGumUpdateMatrix();
        }
       
        sys::sceGumMatrixMode(sys::MatrixMode::Model);
        sys::sceGumLoadIdentity();
        // setup texture

        sys::sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, 1);
        sys::sceGuTexImage(MipmapLevel::None, IMAGE_SIZE as i32, IMAGE_SIZE as i32, IMAGE_SIZE as i32, texture.as_mut_ptr_direct_to_vram() as *const _);
        sys::sceGuTexFunc(TextureEffect::Replace, TextureColorComponent::Rgb);
        sys::sceGuTexFilter(TextureFilter::Nearest, TextureFilter::Nearest);
        sys::sceGuTexScale(1.0/16.0, 1.0/16.0);
        sys::sceGuTexOffset(0.0, 0.0);
        // draw chunk
        psp::sys::sceCtrlReadBufferPositive(pad_data, 1);
        world.render(position);
        //world.process(position);
            

        
        sys::sceGuFinish();
        sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);

        sys::sceDisplayWaitVblankStart();
        sys::sceGuSwapBuffers();
        //val += 1.0;
    }

    sys::sceGuTerm();
    psp::sys::sceKernelExitGame();
}