# Threamon
 Rust Terimal creature capturing and cataloging game
 
# Purpose
This is an example project I did as the leader of my coding club at my high school. I made it as a demo of what we would learn to make.
Feel free to look at the code!

# Dependents
- TerimalRtdm https://crates.io/crates/TerimalRtdm
- Rand

------------
  
# User flow

The user starts on a menu, showing them two options.
- `q 👈 to quit 🏃`
- `p 👈 to play 🎲`
This is done so that the user knows the quit keybind.

Next, the user is prompted to choose a starter
a similar method is used to collect the users choice.
The choice is then pushed to a variable made with the player struct, on a property which is a vec<>

After that flags are ticked so that the player sees the rendered matrix, which is stored on the world struct of vec<vec<>>

Then the game can begin each input the player position is updated and rendered.

A random number is generated and if it matches a saved number the player enters the battle

The battle lets the player flee with a 50-50 chance of dying/losing.
The player is attacked after their move until one dies in the battle.
