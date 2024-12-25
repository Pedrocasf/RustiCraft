use na::{
    Vector2,
    Vector3
};
use crate::vertices::TexCoords;
pub const FACES:[Vector3<f32>;6] = [
    Vector3::<f32>::new(0.0,16.0,0.0),
    Vector3::<f32>::new(0.0,-16.0,0.0),
    Vector3::<f32>::new(-16.0,0.0,0.0),
    Vector3::<f32>::new(16.0,0.0,0.0),
    Vector3::<f32>::new(0.0,0.0,16.0),
    Vector3::<f32>::new(0.0,0.0,-16.0)
];
pub const STONE:TexCoords = TexCoords::tex_coords(
    (0.0,0.0),
    (0.0,0.0),
    (0.0,0.0),
    (0.0,0.0)
);
pub const GRASS:TexCoords = TexCoords::tex_coords(
        (1.0,0.0),
        (2.0,0.0),
        (3.0,0.0),
        (3.0,0.0)
    );
pub const DIRT:TexCoords = TexCoords::tex_coords(
        (2.0,0.0),
        (2.0,0.0),
        (2.0,0.0),
        (2.0,0.0)
    );
pub const BRICK:TexCoords = TexCoords::tex_coords(
        (0.0,1.0),
        (0.0,1.0),
        (0.0,1.),
        (0.0,1.0)
    );
pub const PUMPKIN:TexCoords = TexCoords::tex_coords(
        (6.0,6.0),
        (6.0,7.0),
        (6.0,7.0),
        (7.0,7.0)
    );
pub const ETABLE:TexCoords = TexCoords::tex_coords(
        (6.0,10.0),
        (6.0,11.0),
        (7.0,11.0),
        (7.0,11.0)
    );
pub const SAND:TexCoords = TexCoords::tex_coords(
    (2.0,1.0),
    (2.0,1.0),
    (2.0,1.0),
    (2.0,1.0)
);
