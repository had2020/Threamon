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
<img width="320" alt="Screenshot 2025-03-20 at 1 37 35 PM" src="https://github.com/user-attachments/assets/7be2b3af-2b27-436b-890f-3068ad5e2b02" />

The user starts on a menu, showing them two options.
- `q 👈 to quit 🏃`
- `p 👈 to play 🎲`
This is done so that the user knows the quit keybind.

<img width="320" alt="Screenshot 2025-03-20 at 1 37 44 PM" src="https://github.com/user-attachments/assets/f0a935a1-2de6-477d-8772-98c4ce6101c2" />

Next, the user is prompted to choose a starter
a similar method is used to collect the users choice.
The choice is then pushed to a variable made with the player struct, on a property which is a vec<>

After that flags are ticked so that the player sees the rendered matrix, which is stored on the world struct of vec<vec<>>

Then the game can begin each input the player position is updated and rendered.

<img width="320" alt="Screenshot 2025-03-20 at 1 38 18 PM" src="https://github.com/user-attachments/assets/18bc0048-fef0-41be-a235-83f651200ed2" />


A random number is generated and if it matches a saved number the player enters the battle

<img width="320" alt="Screenshot 2025-03-20 at 1 37 57 PM" src="https://github.com/user-attachments/assets/f6964fee-285b-47db-a6c8-3581597cd31d" />

The battle lets the player flee with a 50-50 chance of dying/losing.
The player is attacked after their move until one dies in the battle.
