use macroquad::rand::ChooseRandom;
use rand::{Rng, seq::SliceRandom};

use crate::member::Member;

pub struct Population{
    
    pub population_vec: Vec<Member>

}

impl Population {
    
    pub fn new<R: Rng+?Sized>(size: usize, member_steps: usize, rng: &mut R) -> Self{
        Self { population_vec:
            
                (0..size)
                    .map(|_|{
                        Member::new_random(member_steps, rng)
                    })
                    .collect()
                            
        }
    }

    //sort members 

     pub fn repopulate<R: Rng+?Sized>(&mut self, difference: usize,  mutation_rate: f32, member_steps: usize, rng: &mut R){


        (0..difference)
            .for_each(|_| {
                let m1 = self.population_vec.choose().unwrap();
                let m2 = self.population_vec.choose().unwrap();
                
                let mut child = m1.breed(m2.clone(), member_steps, rng);
                
                child.mutate(mutation_rate, rng);
                
                self.population_vec.push(child);

            })
    }
}