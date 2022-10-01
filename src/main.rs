use macroq::population::Population;
use macroquad::{prelude::*, window};
use ::rand::Rng; 
use macroq::obstacle::Obstacle; 
use macroq::target::Target; 


fn window_conf() -> Conf {
    Conf {
        window_title: "Window name".to_owned(),
        window_height: 700,
        window_width: 1000,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    

    let mut rng = ::rand::thread_rng();
    let population = Population::new(100, 1000, &mut rng);

    let obstacles: Vec<Obstacle> = vec![Obstacle{height: 400.0, width: 10.0, x_coordinate: 800.0, y_coordinate: 0.0},
                                        Obstacle{height: 300.0, width: 10.0, x_coordinate: 400.0, y_coordinate: 400.0},
                                        Obstacle{height: 500.0, width: 10.0, x_coordinate: 150.0, y_coordinate: 0.0}];

    loop {
        clear_background(BLACK);


        //crea una popolazione di punti blu 
      
        obstacles
            .iter()
            .for_each(|obstacle| draw_rectangle(obstacle.x_coordinate, obstacle.y_coordinate, obstacle.width, obstacle.height, GREEN));
      
        //fixed starting position before x:150 
        population
            .members()
            .iter()
            .for_each(|_| {
                draw_circle(100.0, 350.0, 1.0, BLUE)
            });
        

        draw_circle(920.0, 400.0, 10.0, RED);

        next_frame().await
    }
}
