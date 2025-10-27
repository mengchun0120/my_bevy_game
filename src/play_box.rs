use crate::game_lib::*;
use crate::game_panel::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayBoxRecord(pub Option<PlayBox>);

#[derive(Resource, Debug, Clone)]
pub struct BoxIndex {
    pub type_index: usize,
    pub rotate_index: usize,
}

impl BoxIndex {
    pub fn new(type_index: usize, rotate_index: usize) -> Self {
        Self {
            type_index,
            rotate_index,
        }
    }

    pub fn rotate(&self) -> Self {
        Self::new(
            self.type_index,
            (self.rotate_index + 1) % PLAY_BOX_ROTATE_COUNT,
        )
    }
}

#[derive(Resource)]
pub struct IndexGen {
    type_count: usize,
    rotate_count: usize,
    rng: StdRng,
}

impl IndexGen {
    pub fn new(type_count: usize, rotate_count: usize) -> Self {
        IndexGen {
            type_count,
            rotate_count,
            rng: StdRng::from_os_rng(),
        }
    }

    pub fn rand_box(&mut self) -> BoxIndex {
        let type_index = self.rng.random_range(0..self.type_count);
        let rotate_index = self.rng.random_range(0..self.rotate_count);
        BoxIndex {
            type_index,
            rotate_index,
        }
    }
}

#[derive(Resource, Component, Debug, Clone)]
pub struct BoxPos {
    pub row: i32,
    pub col: i32,
}

impl BoxPos {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[derive(Resource, Debug)]
pub struct PlayBox {
    pub pos: BoxPos,
    pub index: BoxIndex,
    pub entities: Vec<Entity>,
}

impl PlayBox {
    pub fn new(
        g: &mut IndexGen,
        game_lib: &GameLib,
        commands: &mut Commands,
        game_panel: &GamePanel,
    ) -> Option<Self> {
        let index = g.rand_box();
        let Some(pos) = game_panel.init_pos(&index, game_lib) else {
            return None;
        };

        let mut play_box = PlayBox {
            pos,
            index,
            entities: Vec::new(),
        };

        play_box.add_components(game_lib, commands, game_panel);

        Some(play_box)
    }

    pub fn reset(
        &mut self,
        pos: &BoxPos,
        index: &BoxIndex,
        commands: &mut Commands,
        game_lib: &GameLib,
        game_panel: &GamePanel,
    ) {
        self.pos = pos.clone();
        self.index = index.clone();
        let box_pos = game_lib.box_pos(&index);
        let mut it = box_pos.iter();

        for e in self.entities.iter() {
            if let Some(pos) = it.next() {
                let row = self.pos.row + pos.row;
                let col = self.pos.col + pos.col;
                let p = game_lib.panel_pos(row, col);
                let v = game_panel.visibility(row, col);
                let mut entity = commands.entity(e.clone());

                entity.entry::<Transform>()
                    .and_modify(move |mut t| {
                        t.translation.x = p.x;
                        t.translation.y = p.y;
                    });

                entity.entry::<Visibility>()
                    .and_modify(move |mut vis|{
                        *vis.as_mut() = v;
                    });
            }
        }
    }

    pub fn put_in_panel(
        &self,
        game_lib: &GameLib,
        game_panel: &mut GamePanel,
    ) {
        let box_pos = game_lib.box_pos(&self.index);
        let mut it = box_pos.iter();

        for e in self.entities.iter() {
            if let Some(pos) = it.next() {
                let row = self.pos.row + pos.row;
                let col = self.pos.col + pos.col;

                game_panel.put_in_entity(row, col, e.clone());
            }
        }
    }

    fn add_components(
        &mut self,
        game_lib: &GameLib,
        commands: &mut Commands,
        game_panel: &GamePanel,
    ) {
        let config = &game_lib.config;
        let init_pos = game_lib.panel_pos(self.pos.row, self.pos.col);
        let color = &game_lib.box_colors[self.index.type_index];
        let box_span = game_lib.box_span;
        let box_config = &config.box_config;
        let bitmap = box_config.play_box_bitmap(&self.index);
        let mut y = init_pos.y;
        let z = box_config.z;
        let mut row = self.pos.row;

        for r in (0..PLAY_BOX_BITMAP_SIZE).rev() {
            let mut x = init_pos.x;
            let mut col = self.pos.col;
            for c in 0..PLAY_BOX_BITMAP_SIZE {
                if bitmap[r][c] != 0 {
                    let e = commands.spawn((
                        Mesh2d(game_lib.box_mesh.clone()),
                        MeshMaterial2d(color.clone()),
                        Transform::from_xyz(x, y, z),
                        game_panel.visibility(row, col),
                    ));
                    self.entities.push(e.id());
                }
                x += box_span;
                col += 1;
            }
            y += box_span;
            row += 1;
        }
    }
}
