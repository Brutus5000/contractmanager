This is a simple CLI app I'm writing in order to learn Rust. The only reason it's public is so that I can ask other people for help pointing at my code.

- All modifying interactions are persisted on disk as events for now. In the long run they are supposed to make REST calls.
- The application terminates after each interaction.
- Each run the application loads it state by replaying all actions from disk.