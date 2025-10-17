# ⏱️ Flussomodoro

Terminal-based time manager to achieve a state of flow

## 🛠️ Installation

### Cargo

```sh
$ cargo install
```

### Nix

#### Declarative

```nix
environment.systemPackages = [
  inputs.flussomodoro.packages.<arch>.flussomodoro
];
```

#### Imperative

```sh
$ nix profile install github:kludge-cs/flussomodoro
```

## 📝 Usage

```sh
$ flussomodoro --help
```

## 🧩 Development

```sh
$ nix develop # If Nix
$ cargo run
```
