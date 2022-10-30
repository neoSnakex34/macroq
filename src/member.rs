use rand::Rng;

#[derive(Clone)]
pub struct Member{

    // starting_coordinates: (f32, f32),
    pub position: (f32, f32),
    moveset: Vec<Move>,
    pub fitness: f32,

}

#[derive(Copy, Clone)]
pub enum Move {

    Right, 
    Left, 
    Up, 
    Down, 

}

impl  Member {
    
    pub fn new_random <R: Rng+?Sized>(size: usize, rng: &mut R) -> Self{
        Self { 
            moveset:   
                (0..size)
                    .map(|_| {
                        Self::select_random_move(rng)
                    })
                    .collect(),

            position: (100.0, 350.0),
            fitness: f32::INFINITY,
        }
    }

    pub fn select_random_move<R: Rng+?Sized>(rng: &mut R) -> Move{
        let n = rng.gen_range(0..=3);
        match  n {
                0 => Move::Right,
                1 => Move::Left,
                2 => Move::Up,
                3 => Move::Down,
                _ => unreachable!()
            }
        }


    pub fn moveset_mutable(&mut self) -> &mut Vec<Move>{
        &mut self.moveset
    } 

    pub fn moveset(&self) -> &Vec<Move>{
        &self.moveset
    }

    pub fn mutate<R: Rng+?Sized>(&mut self, mutation_rate: f32, rng: &mut R){
      
        self
            .moveset
            .iter_mut()
            .for_each(|m| {
                if rng.gen::<f32>() < mutation_rate{
                    
                }
            });
    }

    pub fn update_position(&mut self, i: usize){

        match self.moveset[i]{

            Move::Up => self.position.1 -= 1.0, 
            Move::Down => self.position.1 += 1.0, 
            Move::Right => self.position.0 += 1.0, 
            Move::Left => self.position.0 -= 1.0, 

        }
    }

    pub fn update_fitness(&mut self, target: &Target) -> f32 {
        self.fitness = cmp()
    }

    pub fn reset_position(&mut self){
        self.position = (100.0, 350.0)
    }

}