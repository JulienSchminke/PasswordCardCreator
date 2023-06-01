extern crate rand;
use rand::{Rng, thread_rng, random};
use rand::SeedableRng;
use rand::rngs::adapter::ReseedingRng;
use rand::rngs::OsRng;
use rand_chacha::{ChaCha20Core, ChaCha20Rng};
use cairo::{ImageSurface, Format, Context, FontSlant, FontWeight, Rectangle};
use std::fs::File;
use rand::prelude::SliceRandom;

const CHARACTER_COUNT: usize = 26;

const COLOR1: (f64, f64, f64) = (66.0, 186.0, 255.0);
const COLOR2: (f64, f64, f64) = (227.0, 202.0, 132.0);
const COLOR3: (f64, f64, f64) = (60.0, 255.0, 156.0);
const COLOR4: (f64, f64, f64) = (5.0, 243.0, 255.0);


fn shuffle_chars(pw_length: usize) -> Vec<char> {
    let all_chars = ('!'..='~').collect::<Vec<char>>();//Alle sichtbaren Zeichen
    let mut random_chars = Vec::new();
    for _character in 0..pw_length*CHARACTER_COUNT {
        random_chars.push(*all_chars.choose(&mut rand::thread_rng()).unwrap());
    }
    return random_chars;
}


fn print_pw_card(pw_card_chars: &Vec<char>) {
    ('A'..='Z').for_each(|c| print!("|{}", c)); //A-Z
    print!("|\n");
    for row in 0..pw_card_chars.len()/CHARACTER_COUNT {
        print!("{}", row+1);
        for character in 0..CHARACTER_COUNT {
            print!("|{}", pw_card_chars[row*CHARACTER_COUNT+character]);
        }
        print!("|{}\n", row+1)
    }
    ('A'..='Z').for_each(|c| print!("|{}", c));
    print!("|\n");
}

fn paint_pw_card(pw_card_chars: &Vec<char>) {
    let surface = ImageSurface::create(Format::Rgb24, 450, 210).unwrap();
    let context = Context::new(&surface).unwrap();
    context.select_font_face("Arial", FontSlant::Normal, FontWeight::Normal);
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint().unwrap();

    //Hintergrundbalken malen
    for row in 0..pw_card_chars.len()/CHARACTER_COUNT+2 {
        context.rectangle(0.0,0.0+15.0*row as f64, 450.0, 15.0);
        if row % 2 == 0 {
            context.set_source_rgba(COLOR1.0/255.0, COLOR1.1/255.0, COLOR1.2/255.0, 1.0);
        } else {
            context.set_source_rgba(COLOR3.0/255.0, COLOR3.1/255.0, COLOR3.2/255.0, 1.0);
        }
        context.fill();
    }

    //context.move_to(20.0, 30.0);
    //context.set_source_rgb(0.0, 0.0, 0.0);
    //context.show_text("Most relation ships seem so transitory");

    context.set_source_rgb(0.0,0.0, 0.0); //schwarz
    context.set_font_size(13.0);

    //Erste Zeile: Spaltenbezeichnungen
    let mut x = 30.0;
    let mut y = 10.0;
    context.move_to(x, y);
    ('A'..='Z').for_each(|c| {
        x += 15.0;
        context.show_text(c.to_string().as_str()).unwrap();
        context.move_to(x, y);
    });

    //2. bis 13. Zeile: Zeichen
    for row in 0..pw_card_chars.len()/CHARACTER_COUNT {
        y += 15.0;
        x = 10.0;
        context.move_to(x, y);
        context.show_text((row+1).to_string().as_str()).unwrap();
        x += 5.0;
        for character in 0..CHARACTER_COUNT {
            x += 15.0;
            context.move_to(x, y);
            context.show_text(pw_card_chars[row*CHARACTER_COUNT+character].to_string().as_str()).unwrap();
        }
        x += 15.0;
        context.move_to(x, y);
        context.show_text((row+1).to_string().as_str()).unwrap();

        //Zeile zeichnen
        context.rectangle(0.0, y, 450.0, 1.0);
        context.stroke();
    }

    //letzte Zeile: Spaltenbezeochnungen
    x = 30.0;
    y += 15.0;
    context.move_to(x, y);
    ('A'..='Z').for_each(|c| {
        x += 15.0;
        context.show_text(c.to_string().as_str()).unwrap();
        context.move_to(x, y);
    });



    let mut png_file = File::create("test.png").unwrap();
    surface.write_to_png(&mut png_file).unwrap();
}

fn paint_pw_card_3digits(pw_card_chars: &Vec<char>) {
    let surface = ImageSurface::create(Format::Rgb24, 365, 155).unwrap();
    let context = Context::new(&surface).unwrap();
    let random_chars = pw_card_chars.into_iter().collect::<String>();

    context.select_font_face("Arial", FontSlant::Normal, FontWeight::Normal);
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint().unwrap();

    context.set_source_rgb(0.0, 0.0, 0.0);
    context.set_font_size(13.0);

    //Erste Zeile Spaltenüberschriften
    let mut x = 35.0;
    let mut y = 10.0;
    context.move_to(x, y);

    let mut alphabet = ('A'..='Z').collect::<String>();
    alphabet.push('_');
    for column_header_index in (0..alphabet.len()).step_by(3) {
        x += 35.0;
        context.show_text(&alphabet[column_header_index..column_header_index+3]).unwrap();
        context.move_to(x, y);
    }
    y += 5.0;
    context.rectangle(0.0, y, 450.0, 1.0);
    context.fill();

    //Zeile 2 - 9:
    let mut current_index = 0;
    for row in 1..=8 {
        y += 15.0;
        x = 10.0;
        context.move_to(x, y);
        context.show_text(row.to_string().as_str()).unwrap();
        x = 0.0;

        for column in 0..9 {
            x += 35.0;
            context.move_to(x, y);
            context.show_text(&random_chars[current_index..current_index+3]).unwrap();
            current_index += 3;
        }

        x += 35.0;
        context.move_to(x, y);
        context.show_text(row.to_string().as_str()).unwrap();

        context.rectangle(0.0, y+2.25, 450.0, 1.0);
        context.fill();
    }
    x = 35.0;
    y += 15.0;
    context.move_to(x, y);

    //Letzte Zeile: Spaltenunterschriften
    for column_header_index in (0..alphabet.len()).step_by(3) {
        println!("{}", &alphabet[column_header_index..column_header_index+3]);
        x += 35.0;
        context.show_text(&alphabet[column_header_index..column_header_index+3]).unwrap();
        context.move_to(x, y);
    }

    //Spaltenbegrenzungen
    x = 30.0;
    for column_line in 0..10 {
        context.rectangle(x, 0.0, 1.0, 155.0);
        context.fill();
        x += 35.0;
    }



    let mut png_file = File::create("test2.png").unwrap();
    surface.write_to_png(&mut png_file).unwrap();
}




fn main() {
    let random_chars = &shuffle_chars(13);
    paint_pw_card_3digits(&random_chars);

}
