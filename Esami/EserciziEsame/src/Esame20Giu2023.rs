use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc::{channel, Receiver, RecvError, Sender, SendError};
use std::sync::{Arc, Condvar, Mutex};

enum ChannelState{
    Open,
    Closed
}
impl PartialEq for ChannelState{
    fn eq(&self, other: &Self)-> bool{
        match (self, other){
            (ChannelState::Closed, ChannelState::Open)=> false,
            (ChannelState::Open, ChannelState::Closed)=> false,
            _ => true
        }
    }
}

pub struct MpMcChannel<E: Send>{ //= come un CHANNEL con N PRODUTTORI + N CONSUMATORI
    data: Mutex<(ChannelState,VecDeque<E>)>, //equivalente di un channel bounded; Buffer circolare
    buffer_size: usize,
    cond : Condvar
}

impl<E: Send > MpMcChannel<E>{
    fn new(n: usize) -> Self{
        let mut buf = VecDeque::with_capacity(n);
        MpMcChannel {
            data: Mutex::new((ChannelState::Open, buf)), //inizialmente il canale è aperto
            buffer_size: n,
            cond : Condvar::new(),
        }
    }
    fn send(&self, e:E) -> Option<()>{
        //chiamo lock 1 sola volta
        let mut try_lock = self.data.lock();
        //se si è verificato un errore interno:
        if try_lock.is_err(){return None}; //se non posso prendere il lock, ritorno None
        //altrimenti, accedo al buffer
        let mut lock = try_lock.unwrap();
        //Attenzione: la chiusura del channel può avvenire anche mentre si è in attesa che si liberi spazio, quindi non posso fare subito UNWRAP
        try_lock = self.cond.wait_while(lock, |l|{l.1.len() == self.buffer_size && l.0 == ChannelState::Open});
        /*while vec.len() == self.buffer_size{ //uguale al WAIT WHILE
            //buffer circolare pieno
            vec = self.cond.wait(vec).unwrap();
        }*/
        if try_lock.is_err(){return None};
        lock = try_lock.unwrap();
        if lock.0 == ChannelState::Closed{return None}; //se il canale è chiuso, ritorno None
        lock.1.push_back(e);
        self.cond.notify_all();
        return Some(());
    }

    fn recv(&self) -> Option<E>{
        let mut try_lock = self.data.lock();
        if try_lock.is_err(){return None};

        let mut lock = try_lock.unwrap();
        //se la chiusura avviene mentre attendo -> blocco tutto
        try_lock = self.cond.wait_while(lock, |tupla| tupla.1.len()==0 && tupla.0 == ChannelState::Open);

        lock = try_lock.unwrap();
        //se all'atto della chiusura sono presenti valori nel buffer
        if lock.0 == ChannelState::Closed && lock.1.len()==0 {return None}
        self.cond.notify_all();
        let next = lock.1.pop_front();
        return next;
    }

    pub fn shutdown(&self) -> Option<()> {
        let try_lock = self.data.lock();
        if try_lock.is_err() {return None}
        let mut lock = try_lock.unwrap();
        lock.0 = ChannelState::Closed;
        self.cond.notify_all();
        return Some(())
    }
}










//CON I CHANNELS
pub struct MpMcChannel2<E: Send> {
    sender: Sender<E>,
    receiver: Mutex<Receiver<E>>,
}
impl<E: Send> MpMcChannel2<E> {
    pub fn new(buffer_size: usize) -> Self {
        let (sender, receiver) = channel();
        MpMcChannel2 {
            sender,
            receiver: Mutex::new(receiver),
        }
    }

    pub fn send(&self, msg: E) -> Result<(), SendError<E>> {
        self.sender.send(msg)
    }

    pub fn recv(&self) -> Result<E, RecvError> {
        let receiver = self.receiver.lock().unwrap();
        receiver.recv()
    }

    pub fn shutdown(&self) {
        drop(self.sender.clone());
    }
}



//IMPLEMENTAZIONE CONDVAR
