# Folder Organizer

This is a small piece of software that I use to organize files that I download. It's open source and free for anybody to use. A small example for its usage can be found in the examples folder. It comes with several features that make it a real useful tool.

## Features

- Multiple matchers that are highly customizable:
  - Write your own JS code to organize your files
  - Pop up a dialog to ask for a new location
  - Remember the last location of a file through hash storage
  - Use regex to customize which matchers are used
- Actions for each file:
  - Move
  - Show (only for macOS)
  - Delete
  - Any combination of the above
- Hash storage for where your files end up
  - Remember where you last moved a file
- File info to use for matching
  - use KMDItemWhereFroms to match based on where the file was downloaded from
- Most importantly: Open source! Customize it to your liking, fork it, add your own matchers, actions, file info, etc. I'd love it if you contributed and created a PR.
- A config file to customize the app
- Logs to keep track of what's happening

### My way of using it:

I use this by compiling the binary and moving it to the folder I want to organize incoming files. I use the Folder Actions feature in macOS to call my program (via AppleScript). The program then organizes the files according to my rules and shows them in Finder. This is especially useful when you want your files both organized and immediately visible to you after download. I suggest using with RegExp Download Manager to maximize your productivity.

## Contributing

### State of Things

There are no tests yet unfortunately. Some documentation is provided throughout the code. Some goals for the future are:

- Write tests
- Write more documentation
- Split binary and library crates (this is already _kind of_ being done)
- Enable better customization
- Add more matchers (reduce dependency on JS matchers)
- Add more actions (not sure what else is needed here, maybe JS actions?, webhooks?, etc.)
- A general refactor might be needed, error reporting is not good at all right now.
- Fix the JS parse/stringify hack.

For now, there isn't really any process for contribution. Simply open a PR and I'll take a look at it. If this project gets more attention, I might consider coming up with a contribution guide in the future. Thanks for your interest.
