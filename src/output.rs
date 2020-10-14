use bounded_vec_deque::BoundedVecDeque;

pub struct OutputBuffer{
    data: BoundedVecDeque<u32>,
    handle: fn(u32, &BoundedVecDeque<u32>) -> ()
}

impl OutputBuffer{
    pub fn new(max_kept_data: usize, handle: fn(u32, &BoundedVecDeque<u32>) -> ()) -> OutputBuffer{
        return OutputBuffer{
            data: BoundedVecDeque::<u32>::new(max_kept_data),
            handle: handle
        }
    }

    pub fn new_printer() -> OutputBuffer{
        return OutputBuffer::new(0, |n, _| println!("{:?}", n));
    }

    pub fn out(&mut self, n: u32){
        (self.handle)(n, &self.data);

        if self.data.max_len() > 0{
            self.data.push_back(n);
        }
    }
}