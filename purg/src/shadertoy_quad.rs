use bevy::{math::{Vec2, vec2}, prelude::Mesh, render::{mesh::Indices, pipeline::PrimitiveTopology}};

/// A rectangle on the XY plane.
#[derive(Debug, Copy, Clone)]
pub struct Quad {
    /// Full width and height of the rectangle.
    pub size: Vec2,
    pub res: Vec2,
}

impl Quad {
    pub fn new(size: Vec2, res : Vec2) -> Self {
        Self { size, res}
    }
}

impl From<Quad> for Mesh {
    fn from(quad: Quad) -> Self {
        let extent_x = quad.size.x / 2.0;
        let extent_y = quad.size.y / 2.0;

        let north_west = vec2(-extent_x, extent_y);
        let north_east = vec2(extent_x, extent_y);
        let south_west = vec2(-extent_x, -extent_y);
        let south_east = vec2(extent_x, -extent_y);
        let vertices = 
            [
                (
                    [south_west.x, south_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 1.0],
                    [0.0,0.0]
                ),
                (
                    [north_west.x, north_west.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0],
                    [0.0,quad.res.y]
                ),
                (
                    [north_east.x, north_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [1.0, 0.0],
                    [quad.res.x,quad.res.y]
                ),
                (
                    [south_east.x, south_east.y, 0.0],
                    [0.0, 0.0, 1.0],
                    [1.0, 1.0],
                    [quad.res.x,0.0]
                ),
            ];

        let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

        let mut positions = Vec::<[f32; 3]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut uvs = Vec::<[f32; 2]>::new();
        let mut frag_coords = Vec::<[f32; 2]>::new();
        for (position, normal, uv, frag_coord) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
            frag_coords.push(*frag_coord);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_attribute("Vertex_Screen", frag_coords);
        mesh
    }
}
