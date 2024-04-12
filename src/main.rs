use std::io::Cursor;
use image::io::Reader as ImageReader;

fn main(){

    // should be write an if statement and the else statement.

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

// https://github.com/JDsProjects/JDBot/blob/0e5d2f5543b2ae0951aeb8824efd51e0da7ec739/utils/image.py#L13
// Rewrite this.


fn crusty(image_bytes : u128) -> u128{

    // what bytes do I pass to this?
    // I need to get bytesIO or bytes to be passed to this function (agh)
    // Return same Image if the image fails.
    // might want to fix local using nightly instead (that may be a problem.)
    // agh

    const LOWEST_SIZE: i32 = 32;
    const HIGHEST_SIZE: i32 = 500;
    
    println!("{}", LOWEST_SIZE);
    println!("{}", HIGHEST_SIZE);
    
    return image_bytes;
    
    //will be changed after this point.
    //idk if pass arg is right.
}

// you don't need invert2 to be written.

fn gadget(text: &str) -> u128{
    return;
    // gadget code
    // https://github.com/JDsProjects/JDBot/blob/0e5d2f5543b2ae0951aeb8824efd51e0da7ec739/utils/image.py#L36
}

fn invert(image_bytes: u128) -> u128{

    // invert bytes and keep gifs as gifs, and other content the same etc.
    return image_bytes;

    // invert 
    // https://github.com/JDsProjects/JDBot/blob/0e5d2f5543b2ae0951aeb8824efd51e0da7ec739/utils/image.py#L85

}

fn call_text(text: str&) -> u128{
    return;

    // https://github.com/JDsProjects/JDBot/blob/0e5d2f5543b2ae0951aeb8824efd51e0da7ec739/utils/image.py#L13
    // call_text stuff.
}

// both laugh stuff

/*
ASSET_SIZE = 220
ASSET_SIZE2 = 300
OFFSET = 10


def laugh_frame(LAUGH_IMAGE: Image.Image, asset: Image.Image) -> Image.Image:
    base = LAUGH_IMAGE.copy()
    base = base.convert("RGBA")
    asset = asset.convert("RGBA")
    asset = asset.resize((ASSET_SIZE, ASSET_SIZE), Image.BICUBIC)
    base.paste(asset, (OFFSET, base.height - (ASSET_SIZE - OFFSET)), asset)
    return base


def laugh(raw_asset: bytes) -> tuple[BytesIO, typing.Literal["gif", "png"]]:
    buff = BytesIO()

    with Image.open("assets/images/laugh.png").convert("RGBA") as template:
        with Image.open(BytesIO(raw_asset)) as asset:
            gif = getattr(asset, "is_animated", False)
            if gif:
                frames = []
                for frame in ImageSequence.Iterator(asset):
                    new_frame = laugh_frame(template, frame.convert("RGBA"))
                    new_frame.info["duration"] = frame.info.get("duration", 0)
                    frames.append(new_frame)

                frames[0].save(buff, format="GIF", save_all=True, append_images=frames[1:], loop=0)
            else:
                laugh_frame(template, asset).save(buff, format="PNG")

    gif = "gif" if gif else "png"

    buff.seek(0)
    return buff, gif


def laugh_frame2(BASE, LAUGH_IMAGE: Image.Image, asset: Image.Image) -> Image.Image:
    base = BASE.copy()
    base = base.convert("RGBA")
    asset = asset.convert("RGBA")
    asset = asset.resize((ASSET_SIZE2, ASSET_SIZE2), Image.BICUBIC)
    base.paste(asset, (OFFSET, base.height - (ASSET_SIZE2 + OFFSET)), asset)
    base.paste(LAUGH_IMAGE, (0, 0), LAUGH_IMAGE)
    return base


def laugh2(raw_asset: bytes) -> tuple[BytesIO, typing.Literal["gif", "png"]]:
    buff = BytesIO()

    with Image.open("assets/images/laugh2.png").convert("RGBA") as template:
        with Image.new("RGBA", (template.width, template.height), "white") as canvas:
            with Image.open(BytesIO(raw_asset)) as asset:
                gif = getattr(asset, "is_animated", False)
                if gif:
                    frames = []
                    for frame in ImageSequence.Iterator(asset):
                        new_frame = laugh_frame2(canvas, template, frame.convert("RGBA"))
                        new_frame.info["duration"] = frame.info.get("duration", 0)
                        frames.append(new_frame)

                    frames[0].save(buff, format="GIF", save_all=True, append_images=frames[1:], loop=0)
                else:
                    laugh_frame2(canvas, template, asset).save(buff, format="PNG")

    gif = "gif" if gif else "png"

    buff.seek(0)
    return buff, gif
*/

/* 
petpet function.
I think that's
https://github.com/JDsProjects/JDBot/blob/master/utils/image.py
Remeber rewrite stuff and delete stuff that isn't releveant ie all functions except invert2 unless you want to try to use two functions.
Invert -> one way
Invert2 - > different way
*/
