use bounded_vec_deque::BoundedVecDeque;

pub struct OutputBuffer<T>
where 
    T: FnMut(u32, &BoundedVecDeque<u32>) -> ()
{
    data: BoundedVecDeque<u32>,
    handle: T
}

impl<T> OutputBuffer<T>
where 
    T: FnMut(u32, &BoundedVecDeque<u32>) -> ()
{
    pub fn new(max_kept_data: usize, handle: T) -> Self{
        return OutputBuffer{
            data: BoundedVecDeque::<u32>::new(max_kept_data),
            handle: handle
        }
    }

    pub fn out(&mut self, n: u32){
        (self.handle)(n, &self.data);

        if self.data.max_len() > 0{
            self.data.push_back(n);
        }
    }
}

#[allow(dead_code)]
pub mod buffers{
    use crate::output::OutputBuffer;
    use bounded_vec_deque::BoundedVecDeque;

    pub fn printer() -> OutputBuffer<impl FnMut(u32, &BoundedVecDeque<u32>) -> ()>{
        return OutputBuffer::new(0, |n, _| println!("{:?}", n));
    }
    
    pub fn storer(container: &mut Vec<u32>) -> OutputBuffer<impl FnMut(u32, &BoundedVecDeque<u32>) -> () + '_>{
        return OutputBuffer::new(0, move |n, _| container.push(n));
    }
}