# Solar Oven

this rust program simulates 26,880 different solar ovens to find the best one for use in my engineering class

materials are defined through macros in src-rs/materials/mod.rs
it uses the awesome [cobyla](https://docs.rs/cobyla) algo to optimize designs

there is a ton of cloning in here that could be removed to increase performance, but it's already fast enough and im lazy.

to run the program, make sure [cargo is installed](https://rustup.rs), then run cargo run --bin optimize --release

Best 3 designs of 26880: 

score: 52.16541348347276 
cost based performance index: 99.8037412669185 
Absorber: Black Construction Paper 
L and W: 9.7134 cm 
H: 10.5988 cm 
Inner Body: Cardboard 
Insulator: R30 Fiberglass 
Insulator Thickness: 4.0467 cm 
Outer Body: Cardboard 
Window: Double Mylar 
Reflectors: Silver Reflective Vinyl 
Reflector Count: 4 
Reflector ML: 3.0000 
Reflector Type: Trapezoidal 
Cost: $1.8458 
Temp: 205.2178°C 

score: 52.72990800164381 
cost based performance index: 77.59866864586618 
Absorber: Black Construction Paper 
L and W: 8.7854 cm 
H: 12.9563 cm 
Inner Body: Cardboard 
Insulator: R30 Fiberglass 
Insulator Thickness: 4.6019 cm 
Outer Body: Cardboard 
Window: Double Mylar 
Reflectors: Mirror Sheets 
Reflector Count: 4 
Reflector ML: 3.0000 
Reflector Type: Trapezoidal 
Cost: $2.3950 
Temp: 206.8502°C 

score: 53.25303116675823 
cost based performance index: 68.57609386778192 
Absorber: Black Construction Paper 
L and W: 8.3837 cm 
H: 14.2274 cm 
Inner Body: Cardboard 
Insulator: R30 Fiberglass 
Insulator Thickness: 4.8919 cm 
Outer Body: Cardboard 
Window: Double Mylar 
Reflectors: S Reflect 2000 
Reflector Count: 4 
Reflector ML: 3.0000 
Reflector Type: Trapezoidal 
Cost: $2.7215 
Temp: 207.6332°C
