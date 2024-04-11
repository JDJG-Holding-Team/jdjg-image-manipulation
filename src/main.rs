use std::io::Cursor;
use image::io::Reader as ImageReader;

fn main(){
    
    //crusty();
    // https://docs.rs/image/latest/image/
    // hmm
    
    // will likely need to take in bytes
    
    /*
    
    def crusty(raw_assets: bytes) -> discord.File:
    f = BytesIO()

    with WImage(blob=raw_assets) as img:
        if img.format in ("GIF",):
            img.coalesce()
            img.iterator_reset()

        for d in (35, 500):
            img.resize(d, d)

        img.save(file=f)
        ext = img.format

    f.seek(0)
    return discord.File(f, f"crusty.{ext}")
    
    */
    
    //goal rewrite that from python to rust will likely only return to bytes though
    
}

fn crusty(image_bytes : u128) -> u128{
    const LOWEST_SIZE: i32 = 32;
    const HIGHEST_SIZE: i32 = 500;
    
    println!("{}", LOWEST_SIZE);
    println!("{}", HIGHEST_SIZE);
    
    return image_bytes;
    
    //will be changed after this point.
    //idk if pass arg is right.
}
