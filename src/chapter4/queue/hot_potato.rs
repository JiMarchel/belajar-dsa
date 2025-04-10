use super::queue::Queue;

pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    // Initialize the queue and enqueue the names
    let mut q = Queue::new(names.len());
    for name in names {
        let _nm = q.enqueue(name);
    }

    while q.len() > 1 {
        //Dequeue and enqueue the names,
        // which simulates passing the potato
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }

        // after num dequeue/enqueue cycles
        // remove one person
        let _rm = q.dequeue();
    }

    q.dequeue().unwrap()
}
