use rand::Rng;

#[derive(Clone)]
pub struct Member{

    // starting_coordinates: (f32, f32),
    moveset: Vec<Move>

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
        Self{ 
            moveset:   
                (0..size)
                    .map(|_| {
                        let n = rng.gen_range(0..=3);
                        match n{
                            0 => Move::Right,
                            1 => Move::Left,
                            2 => Move::Up,
                            3 => Move::Down,
                            _ => unreachable!()
                        }
                    })
                    .collect()
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

}