use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::{imageops, ImageError, ImageFormat};

fn main() {

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

fn crusty(image_bytes: Vec<u8>) -> Result<Vec<u8>, ImageError> {
    const WIDTH: u32 = 32;
    const HEIGHT: u32 = 500;

    let mut final_bytes: Vec<u8> = Vec::new();
    let img = match ImageReader::new(Cursor::new(image_bytes)).with_guessed_format()?.decode() {
        Ok(bytes) => {
            bytes.resize(WIDTH, HEIGHT, imageops::Nearest);
            bytes
        },
        Err(_) => ImageReader::open("/assets/images/bad_output.png")?.decode()?
    };

    Ok(img.into_bytes())
}

// you don't need invert2 to be written.

fn gadget(text: String) -> Vec<u8> {

    // let font = FontRef::try_from_slice(include_bytes!("assets/fonts/verdana_edited.ttf")).unwrap();
    // idk the import for this.

    let mut final_bytes: Vec<u8> = Vec::new();
    // let error_img = ImageReader::open("/assets/images/bad_output.png")?.decode();
    let gadget_img = ImageReader::open("/assets/images/gadget.png")?.decode();

    // gadget_img.write_to(&mut Cursor::new(&mut final_bytes));
    // how do I get the ImageFormat for it?

    // gadget code
    // https://github.com/JDsProjects/JDBot/blob/0e5d2f5543b2ae0951aeb8824efd51e0da7ec739/utils/image.py#L36


    return final_bytes;
    
}

fn invert(image_bytes: Vec<u8>) -> Vec<u8> {
    // invert bytes and keep gifs as gifs, and other content the same etc.

    let img = ImageReader::new(Cursor::new(image_bytes)).with_guessed_format()?.decode(); 
    // idk about inverting????

    return image_bytes;

    // invert
    // https://github.com/JDsProjects/JDBot/blob/0e5d2f5543b2ae0951aeb8824efd51e0da7ec739/utils/image.py#L85
}

fn call_text(text: String) -> Vec<u8> {

    let mut final_bytes: Vec<u8> = Vec::new();
    let call_img = ImageReader::open("/assets/images/calling_template.png")?.decode();

    // call_img.write_to(&mut Cursor::new(&mut final_bytes));
    // how do I get the ImageFormat?

    return final_bytes;
    
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

https://github.com/JDsProjects/RenDev-s-PIL-program
(petpet original might refer to newer copies from people that I can license from ie MIT OR MPL)
This is licensed under the JDJG's Programs project and agreed to allow me to use it under the license that i Have so.

Remeber rewrite stuff and delete stuff that isn't releveant ie all functions except invert2 unless you want to try to use two functions.
Invert -> one way
Invert2 - > different way
Might even try writing pillow code to wand like gadget and whatnot to see how it would be.
But this may work better completly in rust so.
*/
