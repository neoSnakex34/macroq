use macroq::member;
use macroq::population::Population;
use macroquad::{prelude::*, window};
use ::rand::Rng; 
use macroq::obstacle::Obstacle; 
use macroq::target::Target; 

const WIDTH: usize = 1000;
const HEIGHT: usize = 700;
const SIZE: usize = 100;

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

fn distance((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
    let diff1 = x2 - x1;
    let diff2 = y2 - y1; 


    (diff1 * diff1 + diff2 * diff2 ).sqrt()

}

#[macroquad::main(window_conf)]
async fn main() {
    let mut rng = ::rand::thread_rng();
    let mut population = Population::new(SIZE, 1000, &mut rng);

    let obstacles: Vec<Obstacle> = vec![Obstacle{height: 400.0, width: 10.0, x_coordinate: 800.0, y_coordinate: 0.0},
                                        Obstacle{height: 300.0, width: 10.0, x_coordinate: 400.0, y_coordinate: 400.0},
                                        Obstacle{height: 500.0, width: 10.0, x_coordinate: 150.0, y_coordinate: 0.0}];

    let target = Target {position: (920.0, 400.0), radius: 10.0};

    let mut i = 0; 

    loop {
        // pulisce lo sfondo
        clear_background(BLACK);

        // crea una popolazione di membri
        obstacles
            .iter()
            .for_each(|obstacle| draw_rectangle(obstacle.x_coordinate, obstacle.y_coordinate, obstacle.width, obstacle.height, GREEN));
      
        // i viene incrementato ad ogni frame,
        // e poiché per sapere quando finiamo il nostro esperimento
        // dobbiamo tenere conto del valore di frame trascorsi
        // dall'inizio dell'esperimento, quando i raggiunge il massimo
        // di passi che ogni membro può compiere, dobbiamo resettare la
        // posizione dei membri ed analizzare i risultati
        if i == 1000 {
            i = 0;

            // aggiorniamo le fitness
            population
                .population_vec
                .iter_mut()
                .for_each(|member| member.update_fitness(&target));

            // sortiamo per valori di fitness
            population
                .population_vec
                .sort_by(|m1, m2| m1.fitness.partial_cmp(&m2.fitness).unwrap());


            // dimezziamo la popolazione
            population.population_vec.drain(SIZE / 2..);

            // population.repopulate();

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
                let prev_position = member.position;

                draw_circle(prev_position.0, prev_position.1, 1.0, BLUE);

                member.update_position(i); // aggiorno la posizione attuale, senza disegnarlo
                
                // collision check
                if member.position.0 < 0.0 
                    || member.position.0 >= WIDTH as f32
                    || member.position.1 < 0.0
                    || member.position.1 >= HEIGHT as f32
                    || obstacles.iter().any(|obstacle| obstacle.contains(member.position)) // obstacle check
                {
                    member.position = prev_position // se ha sbattuto da qualche parte, resetta la posizione alla posizione precedente
                }
            });
        

        // target
        draw_circle(target.position.0, target.position.1, target.radius, RED);

        // aggiornamento counter dei frame/step dei membri
        i += 1;

        next_frame().await
    }
}
