use std::time::Instant;
pub fn log_request<F,R>(method: &str,path : &str,handler:F)->R
where 
    F:FnOnce()->R,
{
    let start = Instant::now();
    let response = handler();
    let duration = start.elapsed();
    println!("INFO {} {} ,completed in {:?}",method,path,duration);
    response
}