This is a anyrun plugin which allows you to run custom commands.

Config example:
```ron
Config(
  prefix: ":cc",
  map: {
    "obsidian": Entry(
      description: "Launch obsidian",
      exec: "obsidian --enable-features=UseOzonePlatform --ozone-platform=wayland --enable-wayland-ime --wayland-text-input-version=3",
      envs: Some([
        ("LANG", "DE")
      ]),
    ),

    "something-else": Entry(
      description: "Launch something else",
      exec: "blub rofl",
      // shows the output of the process (when anyrun is launched on the command line)
      print_output: Some(true),
    ),
  }
)
```

# Include with homeManager

```nix

# in your anyrun.nix
programs.anyrun = {
  ...
  plugins = [
    ...
    "${inputs.anyrun-custom-command.packages.x86_64-linux.default}/lib/libcustom_command.so"
  ]
};
  
```


