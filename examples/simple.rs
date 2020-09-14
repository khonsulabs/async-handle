use async_handle::Handle;

#[tokio::main]
async fn main() {
    let handle = Handle::new(1u32);
    {
        let read_handle = handle.read().await;
        println!("The value is {}", *read_handle);
    }

    {
        let mut write_handle = handle.write().await;
        *write_handle = 2;
    }

    {
        let read_handle = handle.read().await;
        println!("The value is {}", *read_handle);
    }
}
