# Shell Completions

Follow the instructions specific to your shell to add completions for clarinet using these files:
- **bash**: *clarinet.bash*
- **elvish**: *clarinet.elv*
- **fish**: *clarinet.fish*
- **powershell**: *_clarinet.ps1*
- **zsh**: *_clarinet*

These files are automatically generated by `clap` based on the arguments structs in *src/frontend/cli.rs*. To regenerate them after making changes, use:

```sh
clarinet completions (bash|elvish|fish|powershell|zsh)
```