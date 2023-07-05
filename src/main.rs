use sdl2; // use Simple Directmedia Layer v2
use sdl2::pixels::Color; // use Color without typing the sdl2::pixels first everytime

fn main() -> Result<(), String> { // Returns a Result which is either type Ok(()) or Err(error_mesage_string) the error string will be printed to the terminal
    /*initialize Simple Directmedia Layer*/
    let sdl = sdl2::init()?; // ? opperator only returns the inner data of the result if the result is type Ok(DataType) or stops current fn and returns the string inside of Err(String)

    /* create a handle for the video subsytem */
    let video_subsytem = sdl.video()?;

    /* create a window (this is OS agnostic) */
    let window = video_subsytem.window("Double Pendulum", 800, 800) // get a window builder with our title. window width and hieght dont matter too much since the window is resizable
        .allow_highdpi() // Change the window settings before we build the window.
        .resizable()
        .build() // build the window with our settings for target os
    .map_err(|error| error.to_string())?; // The syntax inside the .map_err() is a function eg: |function_param| function body

    /* create a canvas to draw on. mut is needed since we will change (mutate) the canvas */
    let mut canvas = window.into_canvas() // returns a canvas builder
        .accelerated()// Change the canvas settings before we build the canvas. In this case we want to be GPU accelerated
        .present_vsync() // keeps the framerate at the max refreshrate of user's monitor
        .build() // build the canvas with our settings
    .map_err(|error| error.to_string())?;


    /* fill the canvas with cyan pixels */
    canvas.set_draw_color(Color::CYAN); // set the canvas' draw color
    canvas.clear(); // will set each pixel on the canvas to the draw color


    /* Display the canvas on the window for 100 itterations of the loop */// THIS IS A PLACE HOLDER
    for _i in 0..100 { // 0 ≤ number_of_itterations < 100 the underscore signifies the variable is unused in the code and only exists for readability
        canvas.present(); // display the canvas to the window
    }

    /* main expects a Result to be returned. however it expects the Ok variant of Result to be empty aka the unit type represented with () */
    return Ok(()); // Ok() is a wrapper of the unit type ()
                   // if there were any errors you should return Err(some_error_string)
                   // the ? opperator returns the function early with the error from the left hand side
}
