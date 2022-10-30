use macroquad::rand::ChooseRandom;
use rand::{Rng, seq::SliceRandom};

use crate::member::Member;

pub struct Population{
    
    pub population_vec: Vec<Member>

}

impl Population {
    
    pub fn new<R: Rng+?Sized>(size: usize, member_size: usize, rng: &mut R) -> Self{
        Self { population_vec:
            
                (0..size)
                    .map(|_|{
                        Member::new_random(member_size, rng)
                    })
                    .collect()
                            
        }
    }

    //sort members 

    pub fn breed<R: Rng+?Sized>(p1: Member, p2: Member, member_size: usize, rng: &mut R) -> Member{
        let mut child = Member::new_random(member_size, rng); // TODO: cambia

        child
            .moveset_mutable()
            .iter_mut()
            .zip(p1.moveset().iter()
                    .zip(p2.moveset().iter()))
            .for_each(|(cg, (gp1, gp2))|{
                *cg = if rng.gen() { *gp1 } else { *gp2 }
            }); 

            child

    }

    pub fn repopulate<R: Rng+?Sized>(&mut self, difference: usize, mem_size: usize,  mutation_rate: f32, rng: &mut R){


        (0..difference)
            .for_each(|_| {
                let m1 = self.population_vec.choose().unwrap();
                let m2 = self.population_vec.choose().unwrap();

                //TODO member size is the num of moves, consider changing it
                // let mut new_member = Self::breed(*m1, *m2, mem_size, rng);
                // new_member.mutate(mutation_rate, rng);
                // self.population_vec.push(new_member);
            })
    }
}