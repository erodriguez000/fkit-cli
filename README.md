# FileKit CLI
A CLI for bootstrapping React, React Native, and Tauri apps configured with TypeScipt, React Router/ React Navigation, and React Query. Built in Rust.


# Vite + React + Typescript
Initialize a Vite + React + TypeScript app with React Router and React Query. This command will automatically open the newly created project in VS Code. Specify a name or choose one.

```
fkit react
```

```
fkit react my-app
```

# Expo + React Native + Typescript
Initialize an Expo + React Native + TypeScript app with React Navigation and React Query. This command will automatically open the newly created project in VS Code. Specify a name or choose one.

```
fkit rn
```

```
fkit rn my-app
```

# Tauri + React + Typescript
Initialize a Tauri + React + TypeScript app with React Router and React Query. This command will automatically open the newly created project in VS Code. Specify a name or choose one.

```
fkit tauri
```

```
fkit tauri my-app
```

# Run a development server for Tauri, React, and React Native apps initialized from fkit

```
fkit dev
```

# Manage Node Modules

Get all node_modules folders nested under the cwd, and list with the size of each folder.

```
fkit node get
```

Delete all node_modules folders nested under the cwd, and list with the size of each folder.

*WARNING: this action is permanent, do NOT run it in a directory with node_modules folders you want to keep!

```
fkit node delete
```

# Manage Cargo Target Directories

Get all target folders nested under the cwd, and list with the size of each folder.

```
fkit target get
```

Delete all target folders nested under the cwd, and list with the size of each folder.

*WARNING: this action is permanent, do NOT run it in a directory with target folders you want to keep!

```
fkit node delete
```

### Usage

Get the help menu
```
fkit
```
```
Usage: fkit [COMMAND]

Commands:
  react
          Build new react app from template base
  rn
          Build new react-native app from template base
  tauri
          Build new tauri app from template base
  target
          Manage your Cargo target directories
  node
          Manage your node_module directories
  dev
          Starts hot-reload developement
  help
          Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help information

  -V, --version
          Print version information


```