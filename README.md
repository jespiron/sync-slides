Presenter can advance slides and all connected students receive updates in real-time

## Setup

Generate the slides by running
```
build.sh
```
This injects the script in `template.html` into the slides at `output.html`.

## Usage

First, start the presenter application:
```
cd presenter
cargo run
```

Next, open `output.html` in your browser. In the Console tab of dev tools, you should see "Connected to presenter server".

The presenter can send commands to advance slides, and students will receive updates in real time.

## Testing

To send commands, connect to the presenter server:
```
websocat ws://127.0.0.1:8080
```

Then you can send the following commands
```
next_slide
prev_slide
```
respectively to advance or go back to previous slide.