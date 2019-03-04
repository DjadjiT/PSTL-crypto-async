extern crate crypto;
extern crate rand;

mod encrypt_function;

use threadpool::ThreadPool;
use rand::{OsRng, Rng};
use std::iter::repeat;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};



static nb_worker : usize = 9;

fn main() {
    //creation of a random generator
    let mut random = OsRng::new().expect("Failed to get OS random generator");
    let mut key: Vec<u8> = repeat(0u8).take(16).collect();
    //fill the key with random unsigned int value
    random.fill_bytes(&mut key[..]);
    let pool = ThreadPool::new(nb_worker);
    let (sender, receiver) = channel();
    let mut input_x8 : Arc::new(Mutex::new([u8;128]));
    let mut _ouput : [u8;128];
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..nb_worker {
        let _c = Arc::clone(&counter);
        let _tx = sender.clone();

        pool.execute(move || {
            batched_encryption(&input_x8, _c, _tx, receiver, &key);
        });
    }

}

fn batched_encryption(mut encrypt_input: Arc<Mutex<[u8;128]>>, counter : Arc<Mutex<i32>>, tx : Sender<[u8;128]>, rx : Receiver<[u8;128]>, key: &[u8]){
    let mut cpt = counter.lock().unwrap();
    let num = *cpt;

    if num == (nb_worker -1) as i32 {
        let in_x8 = encrypt_input.lock().unwrap();
        let input_x8 = *in_x8;
        let output: [u8;128] = encrypt_function::encrypt_x8(&input_x8, key);

        //println!("{:?} ", output);
        let sender = tx.clone();
        std::mem::drop(in_x8);
        sender.send(output).unwrap();
    }
    else {
        let mut rng = rand::thread_rng();
        let mut input : [u8;16] = rng.gen();
        let in_x8 = encrypt_input.lock().unwrap();
        let input_x8 = *in_x8;

        let mut i : i32 = 0;
        for j in input.iter() {
            let id : usize = (i*num + i) as usize;
            *input_x8[id] = *j;
            i+=1;
        }
        *cpt += 1;
        std::mem::drop(cpt);
        println!("input {}  : {:?}", num, input);

        let received = rx.recv().unwrap();
        let id = num*16;
        let mut encrypt : [u8; 16] = [0;16];
        let mut i : i32 = 0;
        for j in encrypt.iter() {
            let id = (i*num + i) as usize;
            encrypt[i as usize] = received[id];
            i+=1;
        }
        println!("encrypt {} : {:?}",num,encrypt);
    }
}
