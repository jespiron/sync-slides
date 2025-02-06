Presenter can advance slides and all connected students receive updates in real-time

## Usage

Start the presenter application:
   ```
   cd presenter
   cargo run
   ```

In a separate terminal, start the student application:
   ```
   cd student
   cargo run
   ```

The presenter can send commands to advance slides, and students will receive updates in real time.

```
websocat ws://127.0.0.1:8080
next_slide
```

You should see "Bumped slide number to xx" on the presenter's side, and "Current slide number: xx" on the studnet's side