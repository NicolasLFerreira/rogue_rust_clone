use crate::entities::entity::{Entity, EntityKind};
use crate::game_map::tile::{Tile, TileKind};
use crate::graphics::rendering::cell::Cell;
use crossterm::style::Color;

pub trait Theme {
    fn tile_theme(&self, tile: Tile) -> Cell;
    fn entity_theme(&self, entity: &Entity) -> Cell;
}

// Current ASCII theme for the crossterm_renderer
pub struct AsciiTheme {}

impl Theme for AsciiTheme {
    fn tile_theme(&self, tile: Tile) -> Cell {
        if !tile.visible {
            Cell {
                glyph: ' ',
                foreground: Color::Black,
                background: Color::Black,
            }
        } else {
            Cell {
                // Exhaustive matches for glyphs so every time a new kind is added,
                // rendering is required to be specified
                glyph: match tile.kind {
                    TileKind::Void => ' ',
                    TileKind::Floor => '.',
                    TileKind::Wall => '=',
                    TileKind::Door => '+',
                    TileKind::Corridor => '#',
                },
                foreground: match tile.kind {
                    TileKind::Void => Color::Black,
                    TileKind::Floor => Color::Green,
                    TileKind::Wall => Color::DarkYellow,
                    _ => Color::White,
                },
                background: match tile.kind {
                    TileKind::Void => Color::Black,
                    _ => Color::Black,
                },
            }
        }
    }

    fn entity_theme(&self, entity: &Entity) -> Cell {
        Cell {
            glyph: match entity.kind {
                EntityKind::Player => '@',
                EntityKind::Enemy => '&',
            },
            foreground: match entity.kind {
                EntityKind::Player => Color::Yellow,
                EntityKind::Enemy => Color::DarkRed,
            },
            background: match entity.kind {
                EntityKind::Player => Color::Black,
                EntityKind::Enemy => Color::Black,
            },
        }
    }
}
