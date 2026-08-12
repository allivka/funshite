# funshite

Simple CPU-based real time 3d renderer

## Description
Funshite is a demo app created in education purposes of learning basic 3D rendering.
Funshite lets you pick a 3D model via OBJ file and allows you to control the camera to view the model.
It only renders the outlines of polygons and does it entirely on the cpu.
Funshite also allows to save current view into a PNG image.

## Build and run - Bash
1. Running:
    ```bash
    cargo run #debug
    cargo run --release #release
    ```

2. Building and then running:
    ```bash
    cargo build && ./target/debug/funshite #debug
    cargo build --release && ./target/release/funshite #release
    ```

## Controls

*All the motion is processed relatively to camera position in a right handed coordinate system where -Z is forward, +x is right, +Y is up

### MOTION:
### A - move left
### D - move right
### W - move forward
### S - move backward
### SPACE - move upward
### CTRL - move downward
### 
### ROTATION:
### Up Arrow / LMB + Mouse Up - Rotate Upward
### Down Arrow / LMB + Mouse Down - Rotate Downward
### Left Arrow / LMB + Mouse Left - Rotate to the right
### Right Arrow / LMB + Mouse Right - Rotate to the left
### Q - Rotate Around Z to the left
### E - Rotate around Z to the right
### 
### SPEED:
### PageUp / Mouse Wheel Up - motion speed increase
### PageDown / Mouse Wheel Down - motion sped decrease
### T - reset motion speed
### R - sync rotation speed with motion speed
### 
### OTHERS:
### Comma - decrease outline thickness
### Period - increase outline thickness
### F - save the view as PNG
