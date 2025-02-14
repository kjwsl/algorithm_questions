pub struct Solution;

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        if meetings.is_empty() {
            return 0;
        }
        
        let mut sorted = meetings.clone();
        sorted.sort_unstable_by_key(|m| m[0]);
        
        let mut free_rooms = BinaryHeap::new();
        for room_id in 0..n {
            free_rooms.push(Reverse(room_id)); 
        }
        
        let mut used_rooms = BinaryHeap::new();
        
        let mut usage = vec![0; n as usize];
        
        for mtg in sorted.iter() {
            let start = mtg[0] as i64;
            let end   = mtg[1] as i64;
            let duration = end - start;
            
            // Update used rooms
            while let Some(&Reverse((free_time, rid))) = used_rooms.peek() {
                if free_time <= start {
                    used_rooms.pop();
                    free_rooms.push(Reverse(rid));
                } else {
                    break;
                }
            }
            

            if let Some(Reverse(rid)) = free_rooms.pop() {
                let new_end = start + duration;
                
                // Use the free room
                used_rooms.push(Reverse((new_end, rid)));
                usage[rid as usize] += 1;
                
            } else {
                // No free room available, wait for the used room with the earliest end time.
                let Reverse((earliest_end_time, rid)) = used_rooms.pop().unwrap();
                
                // The meeting is delayed to 'earliest_end_time'
                let new_end = earliest_end_time + duration;
                
                // Use the room
                used_rooms.push(Reverse((new_end, rid)));
                usage[rid as usize] += 1;
            }
        }
        
        let mut answer = 0;
        let mut max_count = usage[0];
        for (i, &count) in usage.iter().enumerate().skip(1) {
            if count > max_count {
                max_count = count;
                answer = i;
            }
        }
        
        answer as i32
    }
}

