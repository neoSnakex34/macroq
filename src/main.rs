use macroq::member;
use macroq::population::Population;
use macroquad::{prelude::*, window};
use ::rand::Rng; 
use macroq::obstacle::Obstacle; 
use macroq::target::Target; 

const WIDTH: usize = 1000;
const HEIGHT: usize = 700;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window name".to_owned(),
        window_height: HEIGHT as i32,
        window_width: WIDTH as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    

    let mut rng = ::rand::thread_rng();
    let mut population = Population::new(100, 1000, &mut rng);

    let obstacles: Vec<Obstacle> = vec![Obstacle{height: 400.0, width: 10.0, x_coordinate: 800.0, y_coordinate: 0.0},
                                        Obstacle{height: 300.0, width: 10.0, x_coordinate: 400.0, y_coordinate: 400.0},
                                        Obstacle{height: 500.0, width: 10.0, x_coordinate: 150.0, y_coordinate: 0.0}];

    let mut i = 0; 

    loop {
        clear_background(BLACK);

        // println!("{}", i);

        //crea una popolazione di punti blu 
      
        obstacles
            .iter()
            .for_each(|obstacle| draw_rectangle(obstacle.x_coordinate, obstacle.y_coordinate, obstacle.width, obstacle.height, GREEN));
      
        if i == 1000 {
            i = 0;

            population
                .population_vec
                .iter_mut()
                .for_each(|m| m.reset_position())
        }
        //fixed starting position before x:150 
        population
            .population_vec
            .iter_mut()
            .for_each(|member| {
                
                let position = member.position;
                draw_circle(position.0, position.1, 1.0, BLUE);
                member.update_position(i);
                
                if member.position.0 < 0.0 
                    || member.position.0 >= WIDTH as f32
                    || member.position.1 < 0.0
                    || member.position.1 >= HEIGHT as f32
                    || obstacles.iter().any(|obstacle| obstacle.contains(member.position)){
                        member.position = position

                }

            });
        

        draw_circle(920.0, 400.0, 10.0, RED);

        i += 1; 
        next_frame().await
    }
}
