use rand::Rng;
use crate::target::Target;

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
                    *m = Member::select_random_move(rng)
                }
            });
    }

    pub fn update_position(&mut self, i: usize){

        match self.moveset[i]{

            Move::Up => self.position.1 -= 5.0, 
            Move::Down => self.position.1 += 5.0, 
            Move::Right => self.position.0 += 5.0, 
            Move::Left => self.position.0 -= 5.0, 

        }
    }

    pub fn update_fitness(&mut self, target: &Target) {
        self.fitness = Self::distance(self.position, target.position)
    }

    pub fn reset_position(&mut self){
        self.position = (100.0, 350.0)
    }
    
    fn distance((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
        let diff1 = x2 - x1;
        let diff2 = y2 - y1; 


        (diff1 * diff1 + diff2 * diff2 ).sqrt()

    }
    
    pub fn breed<R: Rng+?Sized>(&self, p2: Member, member_steps: usize, rng: &mut R) -> Member{
        
        let mut child = Member::new_random(member_steps, rng); // TODO: cambia

        child
            .moveset_mutable()
            .iter_mut()
            .zip(self.moveset().iter()
                    .zip(p2.moveset().iter()))
            .for_each(|(cg, (gp1, gp2))|{
                *cg = if rng.gen() { *gp1 } else { *gp2 }
            }); 

            child

    }



}