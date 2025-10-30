use crate::game_lib::*;
use crate::game_panel::*;
use crate::utils::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct BoxIndex {
    pub type_index: usize,
    pub rotate_index: usize,
}

impl BoxIndex {
    pub fn rotate(&mut self) {
        self.rotate_index = (self.rotate_index + 1) % PLAY_BOX_ROTATE_COUNT;
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

#[derive(Resource, Component, Debug, Clone, Default)]
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
pub struct PlayBoxRegion {
    pub box_origin: Vec2,
    row_count: usize,
    col_count: usize,
}

impl PlayBoxRegion {
    pub fn new(box_origin: Vec2, row_count: usize, col_count: usize) -> Self {
        Self {
            box_origin,
            row_count,
            col_count,
        }
    }

    pub fn get_visibility(&self, row: i32, col: i32) -> Visibility {
        if (0..self.row_count as i32).contains(&row) && (0..self.col_count as i32).contains(&col) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct PlayBox {
    pos: BoxPos,
    index: Option<BoxIndex>,
    entities: Vec<Entity>,
}

impl PlayBox {
    pub fn init(
        &mut self,
        index: BoxIndex,
        pos: BoxPos,
        region: &PlayBoxRegion,
        game_lib: &GameLib,
        commands: &mut Commands,
    ) {
        if self.is_valid() {
            return;
        }

        self.pos = pos;
        self.index = Some(index);
        self.add_components(region, game_lib, commands);
    }

    pub fn move_to(
        &mut self,
        new_pos: BoxPos,
        region: &PlayBoxRegion,
        game_lib: &GameLib,
        commands: &mut Commands,
    ) {
        if !self.is_valid() {
            return;
        }

        self.pos = new_pos;
        self.update_pos_vis(region, commands, game_lib);
    }

    pub fn rotate(
        &mut self,
        region: &PlayBoxRegion,
        game_lib: &GameLib,
        commands: &mut Commands,
    ) {
        if !self.is_valid() {
            return;
        }

        self.index.as_mut().unwrap().rotate();
        self.update_pos_vis(region, commands, game_lib);
    }

    #[inline]
    pub fn pos(&self) -> &BoxPos {
        &self.pos
    }

    #[inline]
    pub fn index(&self) -> Option<&BoxIndex> {
        self.index.as_ref()
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.index.is_some()
    }

    pub fn transfer(
        &mut self,
        source: &mut PlayBox,
        new_pos: BoxPos,
        new_region: &PlayBoxRegion,
        commands: &mut Commands,
        game_lib: &GameLib,
    ) {
        self.pos = new_pos;
        self.index = source.index.take();
        self.transfer_entities(source);
        self.update_pos_vis(new_region, commands, game_lib);
    }

    pub fn put_in_panel(&mut self, game_lib: &GameLib, game_panel: &mut GamePanel) {
        let Some(index) = &self.index else {
            return;
        };
        let box_pos = game_lib.box_pos(index);
        let mut it = box_pos.iter();

        for e in self.entities.iter() {
            if let Some(pos) = it.next() {
                let row = self.pos.row + pos.row;
                let col = self.pos.col + pos.col;

                game_panel.put_in_entity(row, col, e.clone());
            }
        }

        self.index = None;
        self.entities.clear();
    }

    fn add_components(
        &mut self,
        region: &PlayBoxRegion,
        game_lib: &GameLib,
        commands: &mut Commands,
    ) {
        let index = self.index.as_ref().unwrap();
        let config = &game_lib.config;
        let init_pos = get_box_pos(
            &region.box_origin,
            self.pos.row,
            self.pos.col,
            game_lib.box_span,
        );
        let color = &game_lib.box_colors[index.type_index];
        let box_span = game_lib.box_span;
        let box_config = &config.box_config;
        let bitmap = box_config.play_box_bitmap(index);
        let mut y = init_pos.y;
        let z = box_config.z;
        let mut row = self.pos.row;

        self.entities.clear();
        for r in (0..PLAY_BOX_BITMAP_SIZE).rev() {
            let mut x = init_pos.x;
            let mut col = self.pos.col;
            for c in 0..PLAY_BOX_BITMAP_SIZE {
                if bitmap[r][c] != 0 {
                    let e = commands.spawn((
                        Mesh2d(game_lib.box_mesh.clone()),
                        MeshMaterial2d(color.clone()),
                        Transform::from_xyz(x, y, z),
                        region.get_visibility(row, col),
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

    fn transfer_entities(&mut self, source: &mut PlayBox) {
        self.entities.clear();
        for e in source.entities.iter() {
            self.entities.push(e.clone());
        }
        source.entities.clear();
    }

    fn update_pos_vis(&self, region: &PlayBoxRegion, commands: &mut Commands, game_lib: &GameLib) {
        let Some(index) = &self.index else {
            return;
        };
        let box_pos = game_lib.box_pos(index);
        let mut it = box_pos.iter();

        for e in self.entities.iter() {
            if let Some(pos) = it.next() {
                let row = self.pos.row + pos.row;
                let col = self.pos.col + pos.col;
                let p = get_box_pos(&region.box_origin, row, col, game_lib.box_span);
                let v = region.get_visibility(row, col);
                let mut entity = commands.entity(e.clone());

                entity.entry::<Transform>().and_modify(move |mut t| {
                    t.translation.x = p.x;
                    t.translation.y = p.y;
                });

                entity.entry::<Visibility>().and_modify(move |mut vis| {
                    *vis.as_mut() = v;
                });
            }
        }
    }
}
