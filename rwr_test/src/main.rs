use std::{ops::Deref, rc::{Rc,Weak}};
use rand::{thread_rng, Rng};
use core::slice;


const ITER:i32 = 1;
//fn rng_bool()->bool{thread_rng().gen_range(1..10)<3}
struct PseudoBadThread{
    iter_count:i32,
    loop_pos:i32,
    variable:i32,
}
impl PseudoBadThread {
    fn new ()->Self{
        Self{iter_count:0,loop_pos:0,variable:0,}
    }
    fn tick(&mut self, public_number:&mut i32){
        if self.iter_count>=ITER{
            return;
        }
        loop{
            match self.loop_pos {
                0=>{
                    self.variable = *public_number;
                    self.loop_pos = 1;
                    return;
                }
                1=>{
                    self.variable += 1;
                    self.loop_pos = 2;
                    return;
                }
                2=>{
                    *public_number = self.variable;
                    self.loop_pos = 0;
                    self.iter_count+=1;
                    if self.iter_count>=ITER{return};
                    return;
                }
                _=>{unreachable!()}
            }
        }
    }
    fn can_tick(&self)->bool{
        self.iter_count<ITER
    }
}


pub enum RWRThreadPos {
    ToCopyFlagFor1stTime,
    ToCheckFlagFor1stTime,
    ToSetUpFlag,
    ToCopyFlagFor2stTime,
    ToCheckFlagFor2stTime,
    ToCopyPublicNumber,
    ToAdd1,
    ToUpdatePublicNumber,
    ToUnsetFlag,
}
struct PseudoRWRLockThread{
    iter_count:i32,
    loop_pos:RWRThreadPos,
    variable:i32,
    flag:bool,
    other_s_flag:bool,
    to_add_iter_count_in_the_end:bool,
}
impl PseudoRWRLockThread {
    fn new ()->Self{
        Self{iter_count:0,loop_pos:RWRThreadPos::ToCopyFlagFor1stTime,variable:0,
            flag:false,other_s_flag:false,to_add_iter_count_in_the_end:false,}
    }
    fn tick(&mut self, public_number:&mut i32, flag_of_the_other:bool)->&str{
        if self.iter_count>=ITER{
            return "Ended";
        }
        loop{
            match self.loop_pos {
                RWRThreadPos::ToCopyFlagFor1stTime=>{
                    self.other_s_flag = flag_of_the_other;
                    self.loop_pos = RWRThreadPos::ToCheckFlagFor1stTime;
                    return "R1";
                }
                RWRThreadPos::ToCheckFlagFor1stTime=>{
                    if self.other_s_flag{
                        self.loop_pos = RWRThreadPos::ToCopyFlagFor1stTime;
                        return "Ch1 Failed";
                    }
                    self.loop_pos = RWRThreadPos::ToSetUpFlag;
                    return "Ch1 OK";
                }
                RWRThreadPos::ToSetUpFlag=>{
                    self.flag = true;
                    self.loop_pos = RWRThreadPos::ToCopyFlagFor2stTime;
                    return "Set Flag";
                }
                RWRThreadPos::ToCopyFlagFor2stTime=>{
                    self.other_s_flag = flag_of_the_other;
                    self.loop_pos = RWRThreadPos::ToCheckFlagFor2stTime;
                    return "R2";
                }
                RWRThreadPos::ToCheckFlagFor2stTime=>{
                    if self.other_s_flag{
                        self.loop_pos = RWRThreadPos::ToUnsetFlag;
                        return "Ch2 Failed";
                    }
                    self.loop_pos = RWRThreadPos::ToCopyPublicNumber;
                    return "Ch2 OK";
                }
                RWRThreadPos::ToCopyPublicNumber=>{
                    self.variable = *public_number;
                    self.loop_pos = RWRThreadPos::ToAdd1;
                    return "R data";
                }
                RWRThreadPos::ToAdd1=>{
                    self.variable += 1;
                    self.loop_pos = RWRThreadPos::ToUpdatePublicNumber;
                    self.to_add_iter_count_in_the_end = true;
                    return "Added 1";
                }
                RWRThreadPos::ToUpdatePublicNumber=>{
                    *public_number = self.variable;
                    self.loop_pos = RWRThreadPos::ToUnsetFlag;
                    return "Wrote data back";
                }
                RWRThreadPos::ToUnsetFlag=>{
                    self.flag = false;
                    if self.to_add_iter_count_in_the_end{
                        self.to_add_iter_count_in_the_end = false;
                        self.iter_count += 1;
                    }
                    self.loop_pos = RWRThreadPos::ToCopyFlagFor1stTime;
                    return "Unset Flag";
                }
                _=>{unreachable!()}
            }
        }
    }
    fn can_tick(&self)->bool{
        self.iter_count<ITER
    }
}


// struct command_gen_9_2{
//     last_pos:i32,
// }
// impl command_gen_9_2 {
//     fn new()->Self{Self{last_pos:0}}
//     fn boundary(&self)->i32{1<<17}
// }
// impl Iterator for command_gen_9_2 {
//     type Item = Box<[bool;18]>;
//     fn next(&mut self) -> Option<Self::Item>{
//         loop{
//             if self.last_pos>=self.boundary(){return None;}
//             let mut count_of_ones = 0;        
//             for i in 0..17{
//                 if self.last_pos&(1<<i)!=0{count_of_ones+=1;}
//             }       
//             if 9 != count_of_ones{
//                 self.last_pos+=1;
//                 continue;
//             }
//             let mut result:Self::Item = Box::new([false;18]);
//             result[17] = false;
//             for i in 0..17{
//                 result[i] = self.last_pos&(1<<i)!=0;//{true}else{false};
//             }
//             self.last_pos+=1;
//             return Some(result);
//         }
//     }
// }

// fn print_command(command:&[bool;18]){
//     let mut pos = [0;2];
//     let names = ["Copy1","Check1","SetFlag","Copy2","Check2","CopyData",
//             "Add1","UpdateData","UnsetFlag",];
//     let mut continuously = true;
//     let last_thread = if command[0]{0}else{1};
//     for c in command{
//         let index = if *c{0}else{1};
//         print!("T{},{} {}, ",index, pos[index], names[pos[index]]);
//         pos[index]+=1;
//     }
// }
// fn print_command_short(command:&[bool;18]){
//     let mut pos = [0;2];
//     let names = ["Copy1","Check1","SetFlag","Copy2","Check2","CopyData",
//             "Add1","UpdateData","UnsetFlag",];
//     let mut continuously = true;
//     let last_thread = if command[0]{0}else{1};
//     let mut to_print = vec![false;18];
//     for i in 1..17{
//         let ii = i+1;
//         if command[i]!=command[ii]{
//             to_print[i] = true;
//             to_print[ii] = true;
//         }
//     }

//     for (i, c) in command.iter().enumerate(){
//         let index = if *c{0}else{1};
//         if !to_print[i]{
//             pos[index]+=1;
//             continue;
//         }

//         print!("T{},{} {}, ",index, pos[index], names[pos[index]]);
//         pos[index]+=1;
//     }
//     println!()
// }



// fn print_command(command:&Vec<bool>){
//     let mut pos = [0;2];
//     let names = ["Copy1","Check1","SetFlag","Copy2","Check2","CopyData",
//             "Add1","UpdateData","UnsetFlag",];
//     let mut continuously = true;
//     let last_thread = if command[0]{0}else{1};
//     for c in command{
//         let index = if *c{0}else{1};
//         print!("T{},{} {}, ",index, pos[index], names[pos[index]]);
//         pos[index]+=1;
//     }
// }








fn main()
{
    // let mut public_number = 0;
    // let mut threads = vec![PseudoBadThread::new(),PseudoBadThread::new(),];
    // loop {
    //     for _ in 1..20{
    //         let random_number = thread_rng().gen_range(0..2);
    //         threads[random_number].tick(&mut public_number);
    //     }
    //         if threads.iter().all(|th|!th.can_tick()){
    //             break;
    //         }
    //     }

    // println!("normal race condition: result is: {}, it should be {} ",public_number , ITER*2);


    // let mut public_number = 0;
    // let mut threads = vec![PseudoRWRLockThread::new(),PseudoRWRLockThread::new(),];
    // loop {
    //     for _ in 1..20{
    //         let random_number = thread_rng().gen_range(0..2);
    //         //let random_number = 0;
    //         let other_index = 1-random_number;
    //         let other_flag = threads[other_index].flag;
    //         threads[random_number].tick(&mut public_number,other_flag);
    //     }
    //         if threads.iter().all(|th|!th.can_tick()){
    //             break;
    //         }
    //     }
    // println!("RWR Lock race condition: result is: {}, it should be {} ",public_number , ITER*2);

    let mut total_test_count = 0;
    let mut ok_count = 0;
    loop {
        let mut public_number = 0;
        let mut threads = vec![PseudoRWRLockThread::new(),PseudoRWRLockThread::new(),];
        
        let mut ends = vec![false;2];
        let mut log:Vec<String> = vec![];
        loop {
            if ends.iter().all(|the_bool|*the_bool){
                break;
            }
            let mut th_index = thread_rng().gen_range(0..2);
            if ends[0]{th_index = 1;}
            if ends[1]{th_index = 0;}
    
            let mut other_index = 0;
            if 0 == th_index {other_index = 1;}
    
            let other_flag = threads[other_index].flag;
            let output_string = threads[th_index].tick(&mut public_number,
                other_flag).to_string();
    
            if "Ended" == output_string {
                ends[th_index] = true;
            }
            let to_push = format!("T{} {}",th_index, output_string);
            log.push(to_push)
        }

        total_test_count +=1;
        match public_number{
            2=>{
                ok_count +=1;
                if total_test_count == ok_count && total_test_count%50_000 == 0{
                    println!("{} test passes", total_test_count);
                }
            }
            1=>{
                println!("{}", public_number);
                println!("{:?}", log);
                return;
            }
            _=>unreachable!()
        }
    }
    //println!("RWR Lock race condition: result is: {}, it should be {} ",public_number , ITER*2);

}
