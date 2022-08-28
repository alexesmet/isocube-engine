mod worlds;

use worlds::blocks;

use sfml::{graphics::{
        Color, 
        RenderTarget, 
        RenderWindow,
        Texture,
        Image,
        View
    }, 
    window::mouse,
    system::Clock,
    window::{
        Event, 
        Key,
        Style
    }};


const SCREEN_SIZE: (u32, u32) = (800,600);

fn main() {
    let mut window = RenderWindow::new(
        SCREEN_SIZE,
        "Iso-cube engine",
        Style::CLOSE,
        &Default::default()
    );
    window.set_vertical_sync_enabled(true);
    let mut view = View::new((0.0, 0.0).into(), (SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32).into());
    view.zoom(0.5);
    window.set_view(&view);
    

    // Entity setup
    let mut texture = Texture::from_file("resources/cube.png").unwrap();
    texture.set_smooth(false);


    let block_faces = blocks::BlockFaces {
        face_x: &Image::from_file("resources/cube_right.png").unwrap(),
        face_y: &Image::from_file("resources/cube_left.png").unwrap(),
        face_z: &Image::from_file("resources/cube_up.png").unwrap()
    };

    let block = blocks::Block::new((0,0,0).into(), &texture, &block_faces);


    let mut world = worlds::World::new();
    world.add(block);

    let mut clock = Clock::start();

    loop {
        while let Some(event) = window.poll_event() {
            
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => return,
                Event::MouseButtonPressed { button: mouse::Button::Left, x, y } => {
                    let world_coords = window.map_pixel_to_coords((x, y).into(), &view);
                    let mut counter = world.blocks.len();
                    while counter > 0 {
                        counter -= 1;
                        let block = &world.blocks[counter];
                        match block.get_coords_of_clicked_face(&world_coords) {
                            Some(cord) => {
                                let new_coord = block.coords + cord;
                                let new_block = blocks::Block::new(new_coord.into(), &texture, &block_faces);
                                world.add(new_block);
                                break;
                            },
                            None => {}
                        }
                    }

                },
                _ => {}
            }
        }

        let delta_time = clock.restart().as_seconds();

        window.clear(Color::rgb(250,250,250));
        for bck in world.blocks.iter() {
            window.draw(&bck.sprite)
        }
        window.display();

    }



}

