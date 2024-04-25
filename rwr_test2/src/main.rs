use std::thread;
use std::sync::mpsc;
use std::ptr::{read_volatile, write_volatile};

//use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;


struct GlobalData{
    public_number:[i32;1],
    flag0:[bool;1],
    flag1:[bool;1],
    op_per_iter:i32,
}
impl Default for GlobalData {
    fn default()->Self{
        GlobalData{public_number:[0], flag0 :[false], 
            flag1:[false],
            op_per_iter:10_000,
            //op_per_iter:33,
        }
    }
}

static mut GLOBAL:Lazy<GlobalData> = Lazy::new(||{GlobalData::default()}) ;// = Default::default();

fn plain_access(){
    unsafe{
        for _ in 0..GLOBAL.op_per_iter{
            GLOBAL.public_number[0]+=1;
        }
    }
}
fn rwr_access(th_id:i32){
    unsafe{
        let mut public_number = GLOBAL.public_number.as_mut_ptr();
        let mut my_flag = GLOBAL.flag0.as_mut_ptr();
        let mut other_flag = GLOBAL.flag1.as_mut_ptr();
        if 1 == th_id{
            my_flag = GLOBAL.flag1.as_mut_ptr();
            other_flag = GLOBAL.flag0.as_mut_ptr();
        }
        let mut count = 0;
        while count < GLOBAL.op_per_iter{
            if read_volatile(other_flag){
                //println!("T{} failed R1",th_id);
                continue;
            }
            write_volatile(my_flag, true);
            if read_volatile(other_flag){
                write_volatile(my_flag, false);
                //println!("T{} failed R2",th_id);
                continue;
            }
            let temp = read_volatile(public_number);
            write_volatile(public_number, temp+1);
            count+=1;
            //println!("T{} added, temp is {}",th_id, temp);
            write_volatile(my_flag, false);
        }
    }
}

fn main() {
    // let mut ii = [123];
    // let mut pi = ii.as_mut_ptr();
    // unsafe{
    //     let temp = read_volatile(pi);
    //     //*pi = temp+1;
    //     write_volatile(pi, temp+1);
    //     println!("{}", ii[0]);
    //     println!("{}", *pi);
    // }
    // return ;

    unsafe{
        GLOBAL = Default::default();
    }
    let th_handle_0 = thread::spawn(plain_access);
    let th_handle_1 = thread::spawn(plain_access);
    let _ = th_handle_0.join(); 
    let _ = th_handle_1.join(); 
    unsafe{
        println!("Plain access test: result is: {}, it should be {}",GLOBAL.public_number[0],GLOBAL.op_per_iter*2);
    }



    unsafe{
        GLOBAL = Default::default();
    }
    let (tx,rx) = mpsc::channel::<i32>();
    let th_handle_0 = thread::spawn(move||{
        let rx = rx;
        let th_id = rx.recv().unwrap();
        rwr_access(th_id);
    });
    tx.send(0).unwrap();
    let (tx,rx) = mpsc::channel();
    let th_handle_1 = thread::spawn(move||{
        let rx = rx;
        let th_id = rx.recv().unwrap();
        rwr_access(th_id);
    });
    tx.send(1).unwrap();
    let _ = th_handle_0.join(); 
    let _ = th_handle_1.join(); 
    unsafe{
        println!("RWR access test: result is: {}, it should be {}",GLOBAL.public_number[0],GLOBAL.op_per_iter*2);
    }

    



}
