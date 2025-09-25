This is a [anyrun](https://github.com/anyrun-org/anyrun) plugin which allows you to run custom commands.

Config example:
```ron
// custom-commands.ron
Config(
  prefix: ":cc",
  commands: {
    Entry(
      title: "Obsidian",
      description: "Launch obsidian",
      exec: "obsidian --enable-features=UseOzonePlatform --ozone-platform=wayland --enable-wayland-ime --wayland-text-input-version=3",
      envs: Some([
        ("LANG", "DE")
      ]),
    ),

    Entry(
      title: "Something else"
      description: "Launch something else",
      exec: "blub rofl",
      // shows the output of the process (when anyrun is launched on the command line)
      // also sometimes needed for scripts which pipe outputs
      print_output: Some(true),
    ),

    Entry(
      title: "Something nested",
      description: "Something nested",
      subcommands: [
        Entry(
          title: "Nested level 1",
          description: "nested on the first level"
          subcommands: [
            Entry(
              title: "Nested level 2",
              description: "nested on the second level",
              exec: "nested"
            )
          ]
        ),

        Entry(
          title: "Also nested on level 1",
          description: "also nested on the first level"
          exec: "also nested"
        )
      ]
    )
  }
)
```

Default Config:
```ron
Config(
  prefix: ":cc",
  commands: []
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


