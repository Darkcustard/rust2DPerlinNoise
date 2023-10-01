# rust2DPerlinNoise
A Rust crate that creates 2D Perlin Noise.

# Installation

* (OPTIONAL) Clone the repository which will allow you to preview your noisemap as an image.
* Download and copy the src/perlin_2d.rs into your project's src folder.

# Usage

### Previewing a Noisemap.

Once you have cloned the repository, You can edit the src/main.rs file and tweak the octave values and
color settings. 

You can then run the crate using: 
~~~
cargo run --release
~~~

This will then display an animated Noisemap.



### In your project:
~~~
mod perlin_2d;


let mut octaves : Vec<[u64;2]> = Vec::new();
    octaves.push([50,50]);
    octaves.push([100,100]);
    octaves.push([200,200]);
    
let noise_map = perlin_2d::NoiseMap2D::new(octaves);
let val : f64 = noise_map.poll(0.0, 0.0);

~~~

# Documentation

Firstly if you are not familiar with Perlin Noise and would like a deeper understanding
you can look at the following *[Wikipedia article](https://en.wikipedia.org/wiki/Perlin_noise)*.

### Octaves
This crate supports multiple layers of noise that compound to make a more detailed noise map.
The "octaves" Vector in the snippet above represents the resolution of each layer and each subsequent
layer is weighted less than the one before it. I recommend using the Visualization feature of this crate
to get a feel for how these octaves affect the final noise map.

### Polling and Returned Value
The NoiseMap2D Struct has a poll method that expects an *X* and a *Y* value of type f64 between 0 and 1.
The poll method will then return an f64 value between 0 and 1 representing the brightness of the noisemap at that coordinate.
