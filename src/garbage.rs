use rand::{thread_rng, Rng};
use rocket::response::stream::ByteStream;

#[allow(non_snake_case)]
#[derive(FromForm)]
pub struct GarbageOptions {
    #[field(default = 4u16)]
    ckSize: u16,
}

#[get("/garbage?<opts..>")]
pub async fn garbage(opts: GarbageOptions) -> ByteStream![Vec<u8>] {
    let mut random_data: Vec<u8> = vec![];
    let mut rng = thread_rng();

    for _ in 0..(1024 * 1024) {
        random_data.push(rng.gen::<u8>());
    }

    let chunks = match opts.ckSize {
        i if i > 1024 => 1024,
        i => i,
    };

    ByteStream! {
        for _ in 0..chunks {
            yield (&random_data).to_vec();
        }
    }
}

#[get("/garbage.php?<opts..>")]
pub async fn garbage_php(opts: GarbageOptions) -> ByteStream![Vec<u8>] {
    garbage(opts).await
}

#[get("/backend/garbage.php?<opts..>")]
pub async fn backend_garbage_php(opts: GarbageOptions) -> ByteStream![Vec<u8>] {
    garbage(opts).await
}

#[get("/backend/garbage?<opts..>")]
pub async fn backend_garbage(opts: GarbageOptions) -> ByteStream![Vec<u8>] {
    garbage(opts).await
}
