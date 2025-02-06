Presenter can advance slides and all connected students receive updates in real-time

## Usage

First, start the presenter application:
```
cd presenter
cargo run
```

The presenter can send commands to advance slides, and students will receive updates in real time.

# Testing

To send command,
```
websocat ws://127.0.0.1:8080
next_slide
```

You should see "Bumped slide number to xx" on the presenter's side.

## With Slides

Generate the slides by running
```
build.sh
```

Next, open `output.html` in your browser. In the Console tab of dev tools, you should see "Connected to presenter server".

## Without Slides

To test without slides, can start the student program instead:
```
cd student
cargo run
```

After sending `next_slide` command, should see "Current slide number: xx" on the student's side