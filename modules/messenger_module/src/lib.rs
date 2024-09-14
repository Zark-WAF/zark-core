use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::sync::mpsc;
use uuid::Uuid;

// define custom types for clarity and ease of use
// these type aliases make the code more readable and maintainable
type Topic = String;  // represents a subject or category for messages
type Message = Vec<u8>;  // allows for flexible message content (can be converted to/from various formats)
type SubscriberId = Uuid;  // unique identifier for subscribers, using uuid for global uniqueness

// subscriber struct to hold id and message sender
// this structure represents an individual subscriber to a topic
struct Subscriber {
    id: SubscriberId,  // unique identifier for the subscriber
    sender: mpsc::Sender<Message>,  // channel for sending messages to this subscriber
}

// main messenger struct to manage subscribers
// this is the core structure that handles all messaging operations
struct ZarkMessenger {
    subscribers: RwLock<HashMap<Topic, Vec<Subscriber>>>,  // thread-safe storage of subscribers by topic
}

impl ZarkMessenger {
    // create a new zark messenger instance
    // this constructor initializes an empty messenger system
    fn new() -> Self {
        ZarkMessenger {
            subscribers: RwLock::new(HashMap::new()),
        }
    }

    // publish a message to all subscribers of a topic
    // this method broadcasts a message to all subscribers of a given topic
    fn publish(&self, topic: &str, message: &[u8]) {
        let subscribers = self.subscribers.read();  // acquire a read lock to access subscribers
        if let Some(topic_subscribers) = subscribers.get(topic) {
            for subscriber in topic_subscribers {
                let _ = subscriber.sender.try_send(message.to_vec());  // attempt to send message, ignoring errors
            }
        }
    }

    // subscribe to a topic and return subscriber id and message receiver
    // this method adds a new subscriber to a topic and returns necessary information for receiving messages
    fn subscribe(&self, topic: &str) -> (SubscriberId, mpsc::Receiver<Message>) {
        let (sender, receiver) = mpsc::channel(100);  // create a new channel with a buffer of 100 messages
        let subscriber = Subscriber {
            id: Uuid::new_v4(),  // generate a new unique id for the subscriber
            sender,
        };
        let mut subscribers = self.subscribers.write();  // acquire a write lock to modify subscribers
        subscribers
            .entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(subscriber);
        (subscriber.id, receiver)
    }

    // unsubscribe from a topic
    // this method removes a subscriber from a specific topic
    fn unsubscribe(&self, topic: &str, subscriber_id: SubscriberId) {
        let mut subscribers = self.subscribers.write();  // acquire a write lock to modify subscribers
        if let Some(topic_subscribers) = subscribers.get_mut(topic) {
            topic_subscribers.retain(|s| s.id != subscriber_id);  // remove the subscriber with matching id
        }
    }
}

// create a new zark messenger instance and return a pointer to it
// this function is exposed to c and creates a new messenger, returning a raw pointer
#[no_mangle]
pub extern "C" fn zark_messenger_create() -> *mut c_void {
    let messenger = Box::new(ZarkMessenger::new());
    Box::into_raw(messenger) as *mut c_void  // convert the box to a raw pointer
}

// safely destroy a zark messenger instance
// this function is exposed to c and properly deallocates the messenger
#[no_mangle]
pub extern "C" fn zark_messenger_destroy(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr as *mut ZarkMessenger);  // convert raw pointer back to box and drop it
        }
    }
}

// send a message to a specific topic
// this function is exposed to c and allows publishing a message to a topic
#[no_mangle]
pub extern "C" fn zark_messenger_send(ptr: *mut c_void, topic: *const c_char, message: *const c_char) -> bool {
    let messenger = unsafe { &*(ptr as *mut ZarkMessenger) };  // convert raw pointer to reference
    let topic = unsafe { CStr::from_ptr(topic).to_str().unwrap() };  // convert c string to rust str
    let message = unsafe { CStr::from_ptr(message).to_bytes() };  // convert c string to byte slice
    messenger.publish(topic, message);
    true
}

// subscribe to a topic and return a pointer to the subscriber
// this function is exposed to c and allows subscribing to a topic
#[no_mangle]
pub extern "C" fn zark_messenger_subscribe(ptr: *mut c_void, topic: *const c_char) -> *mut c_void {
    let messenger = unsafe { &*(ptr as *mut ZarkMessenger) };  // convert raw pointer to reference
    let topic = unsafe { CStr::from_ptr(topic).to_str().unwrap() };  // convert c string to rust str
    let (id, topic, receiver) = messenger.subscribe(topic);
    let subscriber = Box::new((id, topic, receiver));
    Box::into_raw(subscriber) as *mut c_void  // convert the box to a raw pointer
}

// receive a message for a subscriber
// this function is exposed to c and allows receiving a message for a subscriber
#[no_mangle]
pub extern "C" fn zark_messenger_receive(subscriber_ptr: *mut c_void, buffer: *mut c_char, buffer_size: usize) -> isize {
    let subscriber = unsafe { &mut *(subscriber_ptr as *mut (SubscriberId, Topic, mpsc::Receiver<Message>)) };
    if let Ok(message) = subscriber.2.try_recv() {
        let copy_size = std::cmp::min(message.len(), buffer_size);
        unsafe {
            std::ptr::copy_nonoverlapping(message.as_ptr(), buffer as *mut u8, copy_size);  // copy message to buffer
        }
        copy_size as isize
    } else {
        -1  // indicate no message was received
    }
}

// unsubscribe a subscriber from a topic
// this function is exposed to c and allows unsubscribing from a topic
#[no_mangle]
pub extern "C" fn zark_messenger_unsubscribe(ptr: *mut c_void, subscriber_ptr: *mut c_void) {
    let messenger = unsafe { &*(ptr as *mut ZarkMessenger) };  // convert raw pointer to reference
    let subscriber = unsafe { Box::from_raw(subscriber_ptr as *mut (SubscriberId, Topic, mpsc::Receiver<Message>)) };
    messenger.unsubscribe(&subscriber.1, subscriber.0);
}