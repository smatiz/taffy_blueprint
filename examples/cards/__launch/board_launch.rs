use crate::components::*;

// pub struct BoardLaunch {
//     board: BoardComponent,
//     text_drawer: TextDrawer,
// }

// impl BoardLaunch {
//     pub async fn new() -> Self {
//         let text_drawer = TextDrawer::new(16).await;
//         let board = BoardComponent::new(vec![
//             Character {
//                 class: Class::Mage,
//                 agility: 2,
//                 intelligence: 3,
//                 mana: 6,
//                 strength: 1,
//             },
//             Character {
//                 class: Class::Warrior,
//                 agility: 0,
//                 intelligence: 1,
//                 mana: 0,
//                 strength: 6,
//             },
//             Character {
//                 class: Class::Rogue,
//                 agility: 5,
//                 intelligence: 4,
//                 mana: 0,
//                 strength: 2,
//             },
//         ]);

//         Self { text_drawer, board }
//     }

//     pub fn update(&mut self) {
//         let rects = crate::components::helper_macroquad::wrapped_into_screen_root_node(
//             self.board.layout(&self.text_drawer),
//         );
//         self.board.update(&rects);
//         self.board.draw(&self.text_drawer, &rects);
//     }
// }
