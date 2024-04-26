use queued_rust::{QueueType, Queue, SortedQueue};

fn main() {
    // Creating a regular queue
    let mut queue = Queue::new();

    // Add items to the regular queue
    queue.add(4);
    queue.add(1);
    queue.add(3);
    queue.add(5);
    queue.add(2);

    // Two methods for iterating (.next and .iter)
    println!("Printing items from regular queue");
    // Notice how the items are printed in order of add 4, 1, 3, 5, 2
    while let Some(item) = queue.next() {
        println!("{}. items left: {}", item, queue.len());
    }

    // Creating a sorted queue
    let mut sorted_queue = SortedQueue::new();

    // Add items to the sorted queue
    sorted_queue.add(4);
    sorted_queue.add(1);
    sorted_queue.add(3);
    sorted_queue.add(5);
    sorted_queue.add(2);

    // Notice how the items are printed in order 1, 2, 3, 4, 5
    println!("Printing items from sorted queue");
    for item in sorted_queue {
        println!("{}", item)
    }
}

