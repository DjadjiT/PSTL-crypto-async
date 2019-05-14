extern crate crypto;

use std::error::Error;
use std::fmt;
use std::cmp;
use std::sync::{Arc, RwLock};
//use crypto::aes::{self, KeySize};

enum Status {
    BeingProcessed,
    Completed
}

#[derive(Clone)]
struct Job{
    plaintext: Vec<u8>,
    iv: [u8;16],
    keys: Vec<u8>,
    len: usize
}

struct Receipt {
    ciphertext: Vec<u8>,
    status: Status,
}

struct Manager {
    jobs: Vec<Job>,
    min_len: usize,
    receipts: Vec<Arc<RwLock<Receipt>>>,
}

impl Manager {
    fn new () -> Manager {
        return Manager {
            jobs: Vec::new(),
            min_len: usize::max_value(),
            receipts: Vec::new()
        }
    }

    fn submit_job(&mut self, job: Job) -> Arc<RwLock<Receipt>> {

        let mut submitted = Receipt { ciphertext: Vec::new(),
                                  status: Status::BeingProcessed };

        submitted.ciphertext = vec![0; job.len];

        let p = Arc::new(RwLock::new(submitted));

        self.receipts.push(p.clone());
        self.min_len = cmp::min(self.min_len, job.len);
        self.jobs.push(job);

        if self.jobs.len() == 8 {

            // Batch encryption (faked)
            for (i, job) in self.jobs.iter().enumerate() { // XXX: get rid of clone() here
                fake_encrypt(&job.plaintext,
                             &mut Arc::clone(&self.receipts[i]).write().unwrap().ciphertext,
                             &job.keys,
                             &job.iv,
                             self.min_len);
            }

            for i in 0..self.jobs.len() {
                self.jobs[i].len -= self.min_len;
                if self.jobs[i].len == 0 {
                    Arc::clone(&self.receipts[i]).write().unwrap().status = Status::Completed;
                }
            }

            for i in 0..self.jobs.len() {
                if self.jobs[i].len == 0 {
                    Arc::clone(&self.receipts[i]).write().unwrap().status = Status::Completed;
                }
            }

            // BUG: if *all* the jobs are done, we must reset min_len

            let mut counter : u8 = 0;

            for i in 0..self.jobs.len(){
                if self.jobs[i].len == 0 {
                    counter = counter + 1;
                }
            }

            if counter == 8 {
                self.min_len = usize::max_value();
            }
            println!("min_len : {:?}", self.min_len);

            for i in 0..self.jobs.len(){
                println!("{:?}", self.receipts[i].read().unwrap().ciphertext);
            }
        }

        return p;

    }


}

fn fake_encrypt(input: &[u8], mut output: &mut [u8], key: &[u8], nonce: &[u8], len: usize) {
    for i in 0..len {
      output[i] = input[i] ^ key[i];
    }
}

fn main() {
    let mut _manager = Manager::new();
    let mut input: Vec<u8> = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1].to_vec();
    let iv : [u8;16] = [0;16];
    let in_len : usize = input.len();
    let keys : Vec<u8> = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16].to_vec();

    let job = Job {
        plaintext : input.clone(),
        iv : iv,
        keys : keys,
        len : in_len
    };
    let mut job2 = job.clone();
    input.pop();
    input.push(2);input.push(2);input.push(2);
    job2.plaintext = input.clone();
    job2.len = input.len();
    let mut job3 = job.clone();
    input.pop();
    input.push(3);
    job3.plaintext = input.clone();
    let mut job4 = job.clone();
    input.pop();
    input.push(4);
    job4.plaintext = input.clone();
    let mut job5 = job.clone();
    input.pop();
    input.push(5);
    job5.plaintext = input.clone();
    let mut job6 = job.clone();
    input.pop();
    input.push(6);
    job6.plaintext = input.clone();
    let mut job7 = job.clone();
    input.pop();
    input.push(7);
    job7.plaintext = input.clone();
    let mut job8 = job.clone();
    input.pop();
    input.push(8);
    job8.plaintext = input.clone();

    _manager.submit_job(job);
    _manager.submit_job(job2);
    _manager.submit_job(job3);
    _manager.submit_job(job4);
    _manager.submit_job(job5);
    _manager.submit_job(job6);
    _manager.submit_job(job7);
    _manager.submit_job(job8);

/*
    let mut args = build_Args();
    let mut manager = build_Manager(args);

    let mut input: Vec<char> = Vec::new();
        input.push('a');input.push('a');input.push('a');input.push('a');input.push('a');
        input.push('a');input.push('a');input.push('a');input.push('a');input.push('a');
        input.push('a');input.push('a');input.push('a');input.push('a');

        let mut output: Vec<char> = input.clone();
        let mut keys: Vec<char> = input.clone();
        keys.push('a');keys.push('a');
        let len: u32 = input.len() as u32;

        let mut job: Aes_job = Aes_job {
            plaintext: input.clone(),
            ciphertext: output,
            iv: [0;16],
            len: len,
            keys: keys.clone(),
            status: Status::Idle
        };

        let mut job2 = job.clone();
        keys.pop();
        keys.push('b');
        job2.keys =  keys.clone();
        let mut job3 = job.clone();
        keys.pop();
        keys.push('c');
        job3.keys =  keys.clone();
        let mut job4 = job.clone();
        keys.pop();
        keys.push('d');
        job4.keys =  keys.clone();
        let mut job5 = job.clone();
        keys.pop();
        keys.push('e');
        job5.keys =  keys.clone();
        let mut job6 = job.clone();
        keys.pop();
        keys.push('f');
        job6.keys =  keys.clone();
        let mut job7 = job.clone();
        keys.pop();
        keys.push('g');
        job7.keys =  keys.clone();
        let mut job8 = job.clone();
        keys.pop();
        keys.push('h');
        input.push('b');
        job8.plaintext = input.clone();
        job8.ciphertext = input.clone();
        job8.keys =  keys.clone();

        manager.submit_job(job);
        manager.submit_job(job2);
        manager.submit_job(job3);
        manager.submit_job(job4);
        manager.submit_job(job5);
        manager.submit_job(job6);
        manager.submit_job(job7);
        manager.submit_job(job8);

        /*println!("job1: {:?}", job.ciphertext);
        println!("job2: {:?}", job2.ciphertext);
        println!("job3: {:?}", job3.ciphertext);
        println!("job4: {:?}", job4.ciphertext);
        println!("job5: {:?}", job5.ciphertext);
        println!("job6: {:?}", job6.ciphertext);
        println!("job7: {:?}", job7.ciphertext);
        println!("job8: {:?}", job8.ciphertext);*/
*/
}
