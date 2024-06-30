use crate::TEXTURES_POS;
use crate::rand::{
    //perlin,
    rand
};
use na::{
    Vector2,
    Vector3
};

use hashbrown::{
    HashMap,
};
use psp::{
    Align16,
};
use psp::sys::GeCommand::VertexType;
#[repr(C, align(16))]
pub struct Cube(
    pub Align16<[(Vector2<f32>,Vector3<f32>);36]>
);
impl Cube{
    pub fn cube_vertices(x:f32,y:f32,z:f32,n:f32, tx:TexCoords)->Cube{
        let top0 = tx.0[0].0[0];
        let top1 = tx.0[0].0[1];
        let top2 = tx.0[0].0[2];
        let top3 = tx.0[0].0[3];
        let bot0 = tx.0[1].0[0];
        let bot1 = tx.0[1].0[1];
        let bot2 = tx.0[1].0[2];
        let bot3 = tx.0[1].0[3];
        let s0 = tx.0[2].0[0];
        let s1 = tx.0[2].0[1];
        let s2 = tx.0[2].0[2];
        let s3 = tx.0[2].0[3];
        let f0 = tx.0[3].0[0];
        let f1 = tx.0[3].0[1];
        let f2 = tx.0[3].0[2];
        let f3 = tx.0[3].0[3];
        Cube(Align16([
            (Vector2::<f32>::new(top0.x, top0.y),Vector3::<f32>::new( x-n,y-n, z+n)),//0
            (Vector2::<f32>::new(top1.x, top1.y),Vector3::<f32>::new( x-n,y+n, z+n)),//4
            (Vector2::<f32>::new(top2.x, top2.y),Vector3::<f32>::new( x+n,y+n, z+n)),//5
            (Vector2::<f32>::new(top0.x, top0.y),Vector3::<f32>::new( x-n,y-n, z+n)),//0
            (Vector2::<f32>::new(top2.x, top2.y),Vector3::<f32>::new( x+n,y+n, z+n)),//5
            (Vector2::<f32>::new(top3.x, top3.y),Vector3::<f32>::new( x+n,y-n, z+n)),//1//top
            (Vector2::<f32>::new(bot0.x, bot0.y),Vector3::<f32>::new( x-n,y-n, z-n)),//3
            (Vector2::<f32>::new(bot1.x, bot1.y),Vector3::<f32>::new( x+n,y-n, z-n)),//2
            (Vector2::<f32>::new(bot2.x, bot2.y),Vector3::<f32>::new( x+n,y+n, z-n)),//6
            (Vector2::<f32>::new(bot0.x, bot0.y),Vector3::<f32>::new( x-n,y-n, z-n)),//3
            (Vector2::<f32>::new(bot2.x, bot2.y),Vector3::<f32>::new( x+n,y+n, z-n)),//6
            (Vector2::<f32>::new(bot3.x, bot3.y),Vector3::<f32>::new( x-n,y+n, z-n)),//7//bottom
            (Vector2::<f32>::new(s0.x, s0.y),Vector3::<f32>::new( x+n,y-n,z-n)),//0
            (Vector2::<f32>::new(s1.x, s1.y),Vector3::<f32>::new( x+n,y-n,z+n)),//3
            (Vector2::<f32>::new(s2.x, s2.y),Vector3::<f32>::new( x+n,y+n,z+n)),//7
            (Vector2::<f32>::new(s0.x, s0.y),Vector3::<f32>::new( x+n,y-n,z-n)),//0
            (Vector2::<f32>::new(s2.x, s2.y),Vector3::<f32>::new( x+n,y+n,z+n)),//7
            (Vector2::<f32>::new(s3.x, s3.y),Vector3::<f32>::new( x+n,y+n,z-n)),//4//left
            (Vector2::<f32>::new(s0.x, s0.y),Vector3::<f32>::new( x-n,y-n,z-n)),//0
            (Vector2::<f32>::new(s1.x, s1.y),Vector3::<f32>::new( x-n,y+n,z-n)),//3
            (Vector2::<f32>::new(s2.x, s2.y),Vector3::<f32>::new( x-n,y+n,z+n)),//7
            (Vector2::<f32>::new(s0.x, s0.y),Vector3::<f32>::new( x-n,y-n,z-n)),//0
            (Vector2::<f32>::new(s2.x, s2.y),Vector3::<f32>::new( x-n,y+n,z+n)),//7
            (Vector2::<f32>::new(s3.x, s3.y),Vector3::<f32>::new( x-n,y-n,z+n)),//4//right
            (Vector2::<f32>::new(f0.x, f0.y),Vector3::<f32>::new( x-n,y+n,z-n)),//0
            (Vector2::<f32>::new(f1.x, f1.y),Vector3::<f32>::new( x+n,y+n,z-n)),//1
            (Vector2::<f32>::new(f2.x, f2.y),Vector3::<f32>::new( x+n,y+n,z+n)),//2
            (Vector2::<f32>::new(f0.x, f0.y),Vector3::<f32>::new( x-n,y+n,z-n)),//0
            (Vector2::<f32>::new(f2.x, f2.y),Vector3::<f32>::new( x+n,y+n,z+n)),//2
            (Vector2::<f32>::new(f3.x, f3.y),Vector3::<f32>::new( x-n,y+n,z+n)),//3//front
            (Vector2::<f32>::new(s0.x, s0.y),Vector3::<f32>::new( x-n,y-n,z-n)),//4
            (Vector2::<f32>::new(s1.x, s1.y),Vector3::<f32>::new( x-n,y-n,z+n)),//7
            (Vector2::<f32>::new(s2.x, s2.y),Vector3::<f32>::new( x+n,y-n,z+n)),//6
            (Vector2::<f32>::new(s0.x, s0.y),Vector3::<f32>::new( x-n,y-n,z-n)),//4
            (Vector2::<f32>::new(s2.x, s2.y),Vector3::<f32>::new( x+n,y-n,z+n)),//6
            (Vector2::<f32>::new(s3.x, s3.y),Vector3::<f32>::new( x+n,y-n,z-n)),//5//back
        ]))
    }
}
union Transmute<T: Copy, U: Copy> {
    from: T,
    to: U,
}
#[derive(Copy, Clone)]
pub struct TexCoord(
    pub [Vector2<f32>;4]
);
impl TexCoord{
    pub const fn tex_coord(xy:Vector2<f32>)->TexCoord{
        let xy_t = unsafe {
            Transmute::<Vector2::<f32>, (f32,f32)> { from:xy }.to 
        };
        let dxy = (xy_t.0, xy_t.1);
        TexCoord([
            Vector2::<f32>::new(dxy.0,dxy.1),
            Vector2::<f32>::new(dxy.0 + 1.0, dxy.1),
            Vector2::<f32>::new(dxy.0 + 1.0, dxy.1 + 1.0),
            Vector2::<f32>::new(dxy.0, dxy.1 + 1.0)
        ])
    }
}
pub const SECTOR_SIZE:usize = 16;
pub fn sectorize(v:Vector3<f32>)->Vector3<u8>{
    let vx = v.x / SECTOR_SIZE as f32;
    let vy = v.y / SECTOR_SIZE as f32;
    let vz = v.z / SECTOR_SIZE as f32;
    let v = Vector3::<i32>::new(vx as i32, vy as i32, vz as i32);
    Vector3::<u8>::new((v.x >>4) as u8, (v.y >> 4) as u8, (v.z >> 4) as u8)
}
#[derive(Copy, Clone)]
pub struct TexCoords(
    pub [TexCoord;4]
);
impl TexCoords{
    pub const fn tex_coords(top:Vector2<f32>,bottom:Vector2<f32>,side:Vector2<f32>,front:Vector2<f32>)->TexCoords{
        TexCoords([
            TexCoord::tex_coord(top),
            TexCoord::tex_coord(bottom),
            TexCoord::tex_coord(side),
            TexCoord::tex_coord(front)
        ])
    }
}
#[derive(Clone)]
pub struct World{
    chunks:HashMap<Vector3<u8>,Chunk>,
}
impl World{
    pub fn new()-> World{
        let mut w = World{
            chunks: HashMap::new()
        };
        w.add_chunk(&Vector3::<u8>::new(0,0,0));
        return w;
    }
    pub fn add_chunk(&mut self,k:&Vector3<u8>){
        if self.chunks.contains_key(k){
            return;
        }else{
            self.chunks.insert(*k, Chunk::new(*k));
        }
    }
    pub fn insert_block_in_chunk(&mut self,k:&Vector3<u8>,pos:Vector3<u8>, block:u8){
        let c = self.chunks.get_mut(k);
        if let Some(chunk) = c{
            chunk.add_block(pos, block);
        }else{
            self.add_chunk(k);
            self.chunks.get_mut(k).unwrap().add_block(pos, block);
        }
    }
    pub fn get_current_chunk(&mut self, k:Vector3<f32>)->&mut Chunk{
        let k = &sectorize(k);
        if self.chunks.contains_key(k){
            return self.chunks.get_mut(k).unwrap();
        }else{
            self.add_chunk(k);
            return self.chunks.get_mut(k).unwrap();
        };
    }
    pub fn update_chunks(&mut self, p:Vector3<f32>){
        let p = &sectorize(p);
        self.add_chunk(p);
        if self.chunks.contains_key(p){
            self.chunks.get_mut(p).unwrap().should_remove = false;
        }
    }
    pub fn cleanup_chunks(&mut self){
        self.chunks.retain(|_, v| !(*v).should_remove);
    }
    pub fn reset_chunks(&mut self){
        for (_, v) in self.chunks.iter_mut(){
            (*v).should_remove = true;
        }
    }
    pub fn render(&mut self, p:Vector3<f32>){
        let current_chunk = self.get_current_chunk(p);
        for (x_pos,x) in current_chunk.data.iter().enumerate(){
            for (y_pos,y )in x.iter().enumerate(){
                for (z_pos,z )in y.iter().enumerate(){
                    if let Some(tx) = TEXTURES_POS[*z as usize]{
                        let cube = Cube::cube_vertices(p.x+ x_pos as f32, p.y+ y_pos as f32, p.z+ z_pos as f32, 0.5, tx);
                        draw_chunk(&cube.0.0);
                    }
                }
            }
        }
    }
    pub fn process(&mut self, p:Vector3<f32>){
        self.update_chunks(p);
        self.cleanup_chunks();
        self.reset_chunks();
        self.render(p);
    }
}
pub fn draw_chunk(c:&[(Vector2<f32>,Vector3<f32>)]){
    use psp::sys::{
        self,
        GuPrimitive, 
        VertexType
    };
    use core::ptr;
    unsafe{
        sys::sceGumDrawArray(
            GuPrimitive::Triangles,
            VertexType::TEXTURE_32BITF | VertexType::VERTEX_32BITF,
            c.len() as i32,
            ptr::null_mut(),
            c as *const [(Vector2<f32>,Vector3<f32>)] as  *const _,
        );
    }
}
#[derive(Clone, Copy)]
pub struct Chunk{
    pub should_remove:bool,
    pub data:[[[u8;SECTOR_SIZE];SECTOR_SIZE];SECTOR_SIZE],
}
impl Chunk{
    pub fn new(p:Vector3<u8>)->Chunk{
        let mut c = Chunk{
            should_remove:true,
            data:[[[0;SECTOR_SIZE];SECTOR_SIZE];SECTOR_SIZE]
        };
        for x in 0..SECTOR_SIZE as u8{
            for y in 0..SECTOR_SIZE as u8{
                for z in (0..SECTOR_SIZE as u8).step_by(4){
                    let blocks = rand().to_ne_bytes();
                    let pos0 = Vector3::<u8>::new(x,y,z);
                    c.add_block(pos0,blocks[0]);
                    let pos1 = Vector3::<u8>::new(x,y,z+1);
                    c.add_block(pos1,blocks[1]);
                    let pos2 = Vector3::<u8>::new(x,y,z+2);
                    c.add_block(pos2,blocks[2]);
                    let pos3 = Vector3::<u8>::new(x,y,z+3);
                    c.add_block(pos3,blocks[3]);
                }
            }
        }
        return c;
    }
    pub fn add_block(&mut self,pos:Vector3::<u8>, block:u8){
        self.data[pos.x as usize][pos.y as usize][pos.z as usize] = block;
    }
    pub fn get_block(&mut self,pos:Vector3::<u8>)-> u8{
        return self.data[pos.x as usize][pos.y as usize][pos.z as usize];
    }
}